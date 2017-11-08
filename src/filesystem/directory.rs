use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

use utils::error;
use filesystem::entry::Entry;

#[derive(Serialize)]
pub struct Directory {
    path: String,
    name: String,
    bsize: u64,
    hsize: f64,
    datetime: String,
    timestamp: i64,
    elements: u64,
    bstring: String,
    acro_bstring: String,
    directories: Vec<Entry>,
    files: Vec<Entry>
}

impl Directory {

    /*

    if let Ok(entries) = p.read_dir() {
        for entry in entries {
            if let Ok(entry) = entry {
                let e_path = entry.path();
                let is_hidden = super::is_hidden(&e_path);
                if is_hidden && !do_hidden { continue; }


            }
        }
    }

    */

    /* A Directory without parsing entries */
    pub fn new_root(p: &PathBuf) -> Directory {
        if !p.is_dir() {
            error::err_msg("PathBuf passed as parameter isn't a directory.");
            error::err_msg("You can't make an instance of Directory struct with this PathBuf !");
            error::exit(false);
        }

        Directory {
            path: super::get_path_string(p),
            name: super::get_filename(p),
            bsize: super::get_size(p),
            hsize: 0.0f64,
            datetime: super::get_datetime(p),
            timestamp: super::get_timestamp(p),
            elements: super::get_nb_elements(p),
            directories: Vec::new(),
            files: Vec::new(),
        }
    }

    pub fn new(p: &PathBuf, do_hidden: bool, do_symlink: bool) -> Directory {
        let mut dir = Directory::new_root(p);

        if let Ok(entries) = p.read_dir() {
            for entry in entries {
                if let Ok(entry) = entry {

                    let e_path = entry.path();
                    let is_hidden = super::is_hidden(&e_path);

                    if is_hidden && !do_hidden { continue; }
                    


                    /*
                    if e_path.is_dir() {

                    }
                    else
                    if e_path.is_file() {

                    }
                    */
                    /*else {

                    }*/

                }
            }
        }

        return dir
    }

}
