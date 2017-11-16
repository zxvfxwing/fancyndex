use std::path::PathBuf;
use std::fs::DirEntry;
use utils::error;

use filesystem;
use filesystem::directory::Directory;
use filesystem::file::File;

pub enum SortMethod {
    name,
    time,
    size,
}

pub struct WalkDir {
    path: PathBuf,
    do_hidden: bool,
    do_symlink: bool,
    go_deep: bool,
    unit_mode: bool,
    sort_method: SortMethod,
    sort_ascending: bool
}

impl WalkDir {
    pub fn init(p: &PathBuf) -> WalkDir {
        WalkDir {
            path: p.to_path_buf(),
            do_hidden: false,
            do_symlink: false,
            go_deep: true,
            unit_mode: true,
            sort_method: SortMethod::name,
            sort_ascending: true,
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

    pub fn use_binary_unit(mut self, mode: bool) -> WalkDir {
        self.unit_mode = mode;
        return self;
    }

    pub fn sort_by(mut self, method: SortMethod, mode: bool) -> WalkDir {
        self.sort_method = method;
        self.sort_ascending = mode;
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
                    else
                    if ftype.is_file() {
                        size = filesystem::get_size(&entry.path());
                        elts = 1;
                    }
                    else
                    if self.do_symlink && ftype.is_symlink() {
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
                            Err(e) => {
                                error::err_msg("\nError occurred while trying to read symbolic link :");
                                error::err_msg(entry.path().to_str().unwrap());
                                error::err_msg(&e.to_string()[..]);
                            }
                        }
                    }
                },
                Err(e) => {
                    error::err_msg("\nError occurred while trying to access file type :");
                    error::err_msg(entry.path().to_str().unwrap());
                    error::err_msg(&e.to_string()[..]);
                }
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
                        match entry.file_type() {
                            Ok(ftype) => {
                                if entry.path().is_dir() && self.do_symlink | !ftype.is_symlink() {
                                    let result = self.deep_run(&entry);
                                    vec_dir.push( Directory::new(&entry.path(), result.0, result.1, self.unit_mode) );
                                    size += result.0;
                                    elts += result.1;
                                }
                                else
                                if entry.path().is_file() && self.do_symlink | !ftype.is_symlink() {
                                    let epath = entry.path();
                                    let fsize = filesystem::get_size(&epath);
                                    vec_file.push( File::new(&epath, fsize, self.unit_mode) );
                                    size += fsize;
                                    elts += 1;
                                }
                            },
                            Err(e) => {
                                error::err_msg("\nError occurred while trying to access file type :");
                                error::err_msg(entry.path().to_str().unwrap());
                                error::err_msg(&e.to_string()[..]);
                            }
                        }
                    }
                }
            }
        }

        /* Create here a new Directory struct, which will be parsed to Json or Template (Rocket) */
        let mut dir = Directory::new(&self.path, size, elts, self.unit_mode);
        dir.add_dirs(vec_dir);
        dir.add_files(vec_file);

        match self.sort_method {
            SortMethod::name => {
                if self.sort_ascending  { dir.sort_by_name_ascending();  }
                else                    { dir.sort_by_name_descending(); }
            },
            SortMethod::time => {
                if self.sort_ascending  { dir.sort_by_time_ascending();  }
                else                    { dir.sort_by_time_descending(); }
            },
            SortMethod::size => {
                if self.sort_ascending  { dir.sort_by_size_ascending();  }
                else                    { dir.sort_by_size_descending(); }
            }
        }

        return dir;
    }
}
