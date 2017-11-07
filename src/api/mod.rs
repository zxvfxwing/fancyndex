/* main module for API
*
* Define methods to iterate on dir
* Prepare JSON for Web UI
*
*/

use std::path::PathBuf;
use filesystem;
use walkdir::{WalkDir, DirEntry};

/*
pub fn get_dir_info(p: &PathBuf) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();

    let walker = WalkDir::new(p).follow_links(true)
                                .max_depth(1)
                                .into_iter();

    for entry in walker.filter_entry(|e| !filesystem::is_hidden(e)) {
        if let Ok(entry) = entry {
            entries.push(Entry::new(&entry, true));
        }
    }

    entries
}

pub fn get_dir_full(p: &PathBuf) -> Entry {
    let mut size = 0u64;
    let mut elements = 0u64;

    let walker = WalkDir::new(p).follow_links(true)
                                .into_iter();

    for entry in walker.filter_entry(|e| !filesystem::dir_entry_is_hidden(e)) {
        if let Ok(entry) = entry {

            let new_entry = Entry::new(&entry, true);

            if entry.file_type().is_file() {
                size += new_entry.get_size();
            }

            elements+=1;
        }
    }

    let walker = WalkDir::new(p).max_depth(0).into_iter();

    let mut the_entry = Entry::empty();

    for entry in walker {
        the_entry = Entry::new(&entry.ok().unwrap(), true);
        the_entry.set_size(size, true);
        the_entry.set_elements(elements);
    }

    the_entry
}
*/
