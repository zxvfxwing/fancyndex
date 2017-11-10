/*
*
* WalkDir struct & module
*
* Init a WalkDir object with differents parameters.
* Follow symbolic link (true/false), default = false
* Compute hidden file (true/false), default = false
*
* sort method :
*   - 0: name down
*   - 1: name up
*   - 2: date down
*   - 3: date up
*   - 4: size down
*   - 5: size up
*   default = 0
*
* This object will run (deep or not, depends on depth setup)
* throught directory.
*
*/

use std::path::PathBuf;
use std::fs::DirEntry;
use utils::error;

use filesystem::directory::Directory;
use filesystem::file::File;

pub struct WalkDir {
    path: PathBuf,
    do_hidden: bool,
    do_symlink: bool,
    go_deep: bool,
    unit_mode: bool,
    sort_method: u8,
}

impl WalkDir {

    pub fn init(p: &PathBuf) -> WalkDir {
        WalkDir {
            path: p.to_path_buf(),
            do_hidden: false,
            do_symlink: false,
            go_deep: true,
            unit_mode: true,
            sort_method: 0u8,
        }
    }

    pub fn do_hidden(mut self, mode: bool) -> WalkDir {
        self.do_hidden = mode;
        return self;
    }

    pub fn do_symlink(mut self, mode: bool) -> WalkDir {
        self.do_symlink = mode;
        return self;
    }

    pub fn go_deep(mut self, mode: bool) -> WalkDir {
        self.go_deep = mode;
        return self;
    }

    pub fn binary_unit(mut self, mode: bool) -> WalkDir {
        self.unit_mode = mode;
        return self;
    }

    pub fn sorting_method(mut self, method: u8) -> WalkDir {
        if method > 5 {
            error::err_msg("No sort method associated to this number !");
            error::err_msg("Should be bewteen [0-5].");
            return self;
        }
        self.sort_method = method;
        return self;
    }

    fn is_hidden(entry: &DirEntry) -> bool {
        entry.file_name()
             .to_str()
             .map(|s| s.starts_with("."))
             .unwrap_or(false)
    }

    fn deep_run(&self, entry: &DirEntry) -> (u64, u64) {
        let mut size = 0u64;
        let mut elts = 0u64;

        if self.go_deep && !WalkDir::is_hidden(entry) | self.do_hidden {
            match entry.file_type() {
                Ok(ftype) => {
                    if ftype.is_dir() {
                        if let Ok(entries) = entry.path().read_dir() {
                            for entry in entries {
                                if let Ok(entry) = entry {
                                    let r = self.deep_run(&entry);
                                    size += r.0;
                                    elts += r.1;
                                }
                            }
                        }
                        elts += 1;
                    }

                    if ftype.is_file() {
                        match entry.metadata() {
                            Ok(metadata) => {
                                size = metadata.len();
                                elts = 1u64;
                            },
                            Err(_) => {}
                        }
                    }

                    else if self.do_symlink {
                        match entry.path().read_link() {
                            Ok(link) => {
                                if let Ok(entries) = link.read_dir() {
                                    for entry in entries {
                                        if let Ok(entry) = entry {
                                            let r = self.deep_run(&entry);
                                            size += r.0;
                                            elts += r.1;
                                        }
                                    }
                                }
                            },
                            Err(_) => {}
                        }
                    }
                },
                Err(_) => {}
            }
        }

        return (size, elts);
    }

    pub fn run(&self) -> Directory {

        let mut vec_dir: Vec<Directory> = Vec::new();
        let mut vec_file: Vec<File> = Vec::new();

        let mut size = 0u64;
        let mut elts = 0u64;

        if let Ok(entries) = self.path.read_dir() {
            for entry in entries {
                if let Ok(entry) = entry {
                    if self.do_hidden | !WalkDir::is_hidden(&entry) {
                        let result = self.deep_run(&entry);
                        let epath = entry.path();

                        if epath.is_dir() {
                            vec_dir.push( Directory::new(&epath, result.0, result.1, self.unit_mode) );
                        }
                        else {
                            vec_file.push( File::new(&epath, result.0, self.unit_mode) );
                        }

                        size += result.0;
                        elts += result.1;
                    }
                }
            }
        }

        let mut dir = Directory::new(&self.path, size, elts, self.unit_mode);

        dir.add_dirs(vec_dir);
        dir.add_files(vec_file);

        return dir;
    }
}
