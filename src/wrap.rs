use everything_rs::{Everything, EverythingRequestFlags};
use derive_more::Display;
use crate::util::{human_readable_size, human_readable_date};

// re-export EverythingSort
pub use everything_rs::EverythingSort;

#[derive(Debug, Display, PartialEq, Eq)]
pub enum FileType {
    Dir,
    File,
}
pub struct Item {
    pub file_type: FileType,
    pub full_path: String,
    pub directory: String,
    pub file_name: String,
    pub size: u64,
    pub size_human: String,
    pub date_created: u64,
    pub date_created_human: String,
    pub date_modified: u64,
    pub date_modified_human: String,
}

pub fn search(query: &str, sort_by: EverythingSort) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    let everything = Everything::new();

    everything.set_search(query);

    everything.set_request_flags(
        EverythingRequestFlags::FullPathAndFileName
        | EverythingRequestFlags::Size
        | EverythingRequestFlags::DateCreated
        | EverythingRequestFlags::DateModified,
    );

    everything.set_sort(sort_by);
    everything.query()?;

    let item_count = everything.get_num_results();
    let mut items = Vec::with_capacity(item_count as usize);

    for (i, path) in everything.full_path_iter().flatten().enumerate() {
        let an_item = Item {
            file_type: if std::path::Path::new(&path).is_dir() {
                FileType::Dir
            } else {
                FileType::File
            },
            full_path: path.clone(),
            directory: std::path::Path::new(&path).parent().unwrap().to_str().unwrap().to_string(),
            file_name: std::path::Path::new(&path).file_name().unwrap().to_str().unwrap().to_string(),
            size: everything.get_result_size(i as u32)?,
            size_human: human_readable_size(everything.get_result_size(i as u32)?),
            date_created: everything.get_result_created_date(i as u32)?,
            date_created_human: human_readable_date(everything.get_result_created_date(i as u32)?),
            date_modified: everything.get_result_count_modified_date(i as u32)?,
            date_modified_human: human_readable_date(everything.get_result_count_modified_date(i as u32)?),
        };
        items.push(an_item);
    }

    Ok(items)
}

pub fn search_by_pa(query: &str) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    search(query, EverythingSort::PathAscending)
}

pub fn search_by_pd(query: &str) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    search(query, EverythingSort::PathDescending)
}

pub fn search_by_mda(query: &str) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    search(query, EverythingSort::DateModifiedAscending)
}

pub fn search_by_mdd(query: &str) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    search(query, EverythingSort::DateModifiedDescending)
}

pub fn search_by_cda(query: &str) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    search(query, EverythingSort::DateCreatedAscending)
}

pub fn search_by_cdd(query: &str) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    search(query, EverythingSort::DateCreatedDescending)
}

pub fn search_by_sa(query: &str) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    search(query, EverythingSort::SizeAscending)
}

pub fn search_by_sd(query: &str) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    search(query, EverythingSort::SizeDescending)
}

// Only for debugging purposes
pub fn show_results(items: Vec<Item>) {
    for (i,item) in items.iter().enumerate() {
        println!(
            "{:>4}: {} {:<100} ({:<80}, {:<20}) {:>20} {} (Created: {})", 
            i, 
            item.file_type,
            item.full_path, 
            item.directory, 
            item.file_name,
            item.size_human,
            item.date_modified_human,
            item.date_created_human,
        );
    }
}
