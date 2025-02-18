use everything_rs::{Everything, EverythingRequestFlags};
use everything_wrapper::util::{human_readable_date, human_readable_size};
use everything_wrapper::wrap::{search, show_results, EverythingSort};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let everything = Everything::new();

    //everything.set_search("report");
    everything.set_search("data c:\\ ext:log");
    //everything.set_search("ext:log");

    everything.set_request_flags(
        EverythingRequestFlags::FullPathAndFileName
            | EverythingRequestFlags::Size
            | EverythingRequestFlags::DateCreated
            | EverythingRequestFlags::DateModified,
    );

    everything.set_sort(EverythingSort::PathAscending);
    everything.query()?;

    println!("Results: |{:<20}||{:>20}|", 1, 2);
    // Iterate over all found results.
    for (i, path) in everything.full_path_iter().flatten().enumerate() {
        let directory = std::path::Path::new(&path)
            .parent()
            .unwrap()
            .to_str()
            .unwrap();
        let file_name = std::path::Path::new(&path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        // Check whether the path is a directory or a file.
        let file_type = if std::path::Path::new(&path).is_dir() {
            "D"
        } else {
            "F"
        };

        let size = everything.get_result_size(i as u32)?;
        let date_created = everything.get_result_created_date(i as u32)?;
        let date_modified = everything.get_result_count_modified_date(i as u32)?;
        println!(
            "{:>4}: {} {:<100} ({:<80}, {:<20}) {:>20} {} (Created: {})",
            i,
            file_type,
            path,
            directory,
            file_name,
            human_readable_size(size),
            human_readable_date(date_modified),
            human_readable_date(date_created)
        );
    }

    let result = search("data c:\\",EverythingSort::PathAscending, 6);
    show_results(result.unwrap());

    Ok(())
}
