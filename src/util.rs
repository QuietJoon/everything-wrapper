use chrono::{Local, LocalResult, TimeZone};

use crate::wrap::{CompactItem, Item, ItemType};

pub fn human_readable_size(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = KIB * 1024;
    const GIB: u64 = MIB * 1024;

    if bytes >= GIB {
        format!("{:.2} GB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.2} MB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.2} KB", bytes as f64 / KIB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn human_readable_date(timestamp: u64) -> String {
    // Convert Windows file time to Unix time:
    // Windows file time is the number of 100-ns intervals since January 1, 1601.
    // Unix time is seconds since January 1, 1970.
    const WINDOWS_TO_UNIX_OFFSET: u64 = 116444736000000000;
    if timestamp < WINDOWS_TO_UNIX_OFFSET {
        return "Invalid date".to_string();
    }
    let unix_timestamp = (timestamp - WINDOWS_TO_UNIX_OFFSET) / 10_000_000;
    match Local.timestamp_opt(unix_timestamp as i64, 0) {
        LocalResult::Single(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        LocalResult::Ambiguous(dt1, _dt2) => dt1.format("%Y-%m-%d %H:%M:%S").to_string(),
        LocalResult::None => "Invalid date".to_string(),
    }
}

pub fn convert_compact_item_to_item(c_item: &CompactItem) -> Item {
    Item {
        item_type: c_item.item_type.clone(),
        full_path: c_item.full_path.clone(),
        directory: std::path::Path::new(&c_item.full_path)
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        file_name: std::path::Path::new(&c_item.full_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        file_extension: if c_item.item_type == ItemType::Dir {
            "Folder".to_string()
        } else {
            let o_extension = std::path::Path::new(&c_item.full_path).extension();
            if o_extension.is_none() {
                "".to_string()
            } else {
                o_extension.unwrap().to_str().unwrap().to_string()
            }
        },
        size: c_item.size,
        size_human: human_readable_size(c_item.size),
        date_created: c_item.date_created,
        date_created_human: human_readable_date(c_item.date_created),
        date_modified: c_item.date_modified,
        date_modified_human: human_readable_date(c_item.date_modified),
    }
}

pub fn convert_compat_item_array_to_item_array(c_items: Vec<CompactItem>) -> Vec<Item> {
    c_items
        .iter()
        .map(|c_item| convert_compact_item_to_item(c_item))
        .collect()
}
