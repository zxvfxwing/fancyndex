use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

use utils::error;
use filesystem::entry::Entry;

#[derive(Serialize)]
pub struct Directory {
    name: String,
    size: u64,
    datetime: String,
    timestamp: i64,
    elements: u64,
    directories: Vec<Entry>,
    files: Vec<Entry>
}

impl Directory {

    /* A Directory without parsing entries */
    pub fn new_root(p: &PathBuf) -> Directory {
        if !p.is_dir() {
            error::err_msg("PathBuf passed as parameter isn't a directory.");
            error::err_msg("You can't make an instance of Directory struct with this PathBuf !");
            error::exit(false);
        }

        Directory {
            name: super::get_filename(p),
            size: super::get_size(p),
            datetime: super::get_datetime(p),
            timestamp: super::get_timestamp(p),
            elements: 0u64,
            directories: Vec::new(),
            files: Vec::new(),
        }
    }

    pub fn new(p: &PathBuf, do_hidden: bool) -> Directory {
        let mut dir = Directory::new_root(p);

        if let Ok(entries) = p.read_dir() {
            for entry in entries {
                if let Ok(entry) = entry {

                    let e_path = entry.path();
                    let is_hidden = super::is_hidden(&e_path);

                    if is_hidden && !do_hidden { continue; }

                    if e_path.is_dir() {


                    }
                    else {

                    }
                }
            }
        }

        return dir
    }

    fn is_hidden(entry: &DirEntry) -> bool {
        entry.file_name()
             .to_str()
             .map(|s| s.starts_with("."))
             .unwrap_or(false)
    }

    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    pub fn set_elts(&mut self, elts: u64) {
        self.elements = elts;
    }

    pub fn run(path: &PathBuf) -> (u64, u64) {

        let do_hidden = true;
        let do_symlink = true;

        let mut size = 0u64;
        let mut elts = 0u64;
        let walker = WalkDir::new(path) .follow_links(do_symlink)
                                        .into_iter();

        /*
        *
        * The predicate is applied to all entries.
        * If the predicate is true, iteration carries on as normal.
        * If the predicate is false, the entry is ignored and if it is a directory,
        * it is not descended into.
        *
        * !is_hidden | do_hidden
        *    true    |    true    => true
        *   false    |    true    => true
        *   false    |   false    => false (here filter does his job)
        *    true    |   false    => true
        *
        */

        for entry in walker.filter_entry(|e| !Directory::is_hidden(e) | do_hidden ) {
            if let Ok(entry) = entry {
                println!("{}", entry.path().display());
                if entry.file_type().is_file() {
                    size += entry.metadata().unwrap().len();
                }
                elts+=1;
                //println!("{}", entry.path().display());
            }
        }

        (size, elts)
    }
}
