use crate::util::{human_readable_date, human_readable_size};
use derive_more::Display;
use everything_rs::{Everything, EverythingRequestFlags};

// re-export EverythingSort
pub use everything_rs::EverythingSort;
pub use everything_sys_bindgen::DWORD;

#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub enum FileType {
    Dir,
    File,
}
#[derive(Clone, Debug)]
pub struct Item {
    pub file_type: FileType,
    pub full_path: String,
    pub directory: String,
    pub file_name: String,
    pub file_extension: String,
    pub size: u64,
    pub size_human: String,
    pub date_created: u64,
    pub date_created_human: String,
    pub date_modified: u64,
    pub date_modified_human: String,
}

pub fn search(
    query: &str,
    sort_by: EverythingSort,
    search_max_limit: u32,
) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    let everything = Everything::new();

    everything.set_search(query);

    everything.set_request_flags(
        EverythingRequestFlags::FullPathAndFileName
            | EverythingRequestFlags::Size
            | EverythingRequestFlags::DateCreated
            | EverythingRequestFlags::DateModified,
    );

    everything.set_sort(sort_by);
    everything.set_max_results(search_max_limit as DWORD);
    everything.query()?;

    let item_count = everything.get_num_results();
    let mut items = Vec::with_capacity(item_count as usize);

    for (i, path) in everything.full_path_iter().flatten().enumerate() {
        let file_type = if std::path::Path::new(&path).is_dir() {
            FileType::Dir
        } else {
            FileType::File
        };
        let o_extension = std::path::Path::new(&path).extension();
        let extension = if file_type == FileType::Dir {
            "Folder".to_string()
        } else {
            if o_extension.is_none() {
                "".to_string()
            } else {
                o_extension.unwrap().to_str().unwrap().to_string()
            }
        };
        let an_item = Item {
            file_type: file_type.clone(),
            full_path: path.clone(),
            directory: std::path::Path::new(&path)
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            file_name: std::path::Path::new(&path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            file_extension: extension,
            size: everything.get_result_size(i as u32)?,
            size_human: human_readable_size(everything.get_result_size(i as u32)?),
            date_created: everything.get_result_created_date(i as u32)?,
            date_created_human: human_readable_date(everything.get_result_created_date(i as u32)?),
            date_modified: everything.get_result_count_modified_date(i as u32)?,
            date_modified_human: human_readable_date(
                everything.get_result_count_modified_date(i as u32)?,
            ),
        };
        items.push(an_item);
    }

    Ok(items)
}

pub fn search_by_pa(query: &str, search_max_limit: u32) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    search(query, EverythingSort::PathAscending, search_max_limit)
}

pub fn search_by_pd(query: &str, search_max_limit: u32) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    search(query, EverythingSort::PathDescending, search_max_limit)
}

pub fn search_by_mda(query: &str, search_max_limit: u32) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    search(query, EverythingSort::DateModifiedAscending, search_max_limit)
}

pub fn search_by_mdd(query: &str, search_max_limit: u32) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    search(query, EverythingSort::DateModifiedDescending, search_max_limit)
}

pub fn search_by_cda(query: &str, search_max_limit: u32) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    search(query, EverythingSort::DateCreatedAscending, search_max_limit)
}

pub fn search_by_cdd(query: &str, search_max_limit: u32) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    search(query, EverythingSort::DateCreatedDescending, search_max_limit)
}

pub fn search_by_sa(query: &str, search_max_limit: u32) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    search(query, EverythingSort::SizeAscending, search_max_limit)
}

pub fn search_by_sd(query: &str, search_max_limit: u32) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    search(query, EverythingSort::SizeDescending, search_max_limit)
}

// Only for debugging purposes
pub fn show_results(items: Vec<Item>) {
    for (i, item) in items.iter().enumerate() {
        println!(
            "{:>4}: {} {:<100} ({:<80}, {:<20}, {:<8}) {:>20} {} (Created: {})",
            i,
            item.file_type,
            item.full_path,
            item.directory,
            item.file_name,
            item.file_extension,
            item.size_human,
            item.date_modified_human,
            item.date_created_human,
        );
    }
}
