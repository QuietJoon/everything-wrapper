use chrono::{Local, LocalResult, TimeZone};

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
