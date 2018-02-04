use std::path::PathBuf;
use std::io::Error;

use filesystem::entries::{Entries, Entry};
use config::EntriesOpt;

use filesystem::pbuf_is_dir;

pub struct WalkDir {
    path: PathBuf,
    hidden: bool,
    symlink: bool,
    e_opt: EntriesOpt,
}

pub struct WalkDirBuilder {
    path: PathBuf,
    hidden: Option<bool>,
    symlink: Option<bool>,
    e_opt: Option<EntriesOpt>,
}

impl WalkDirBuilder {

    pub fn new(path: PathBuf) -> Self {
        WalkDirBuilder {
            path,
            hidden: None,
            symlink: None,
            e_opt: None,
        }
    }

    pub fn do_hidden(&mut self, hidden: bool) -> &mut Self {
        self.hidden = Some(hidden);
        self
    }

    pub fn do_symlink(&mut self, symlink: bool) -> &mut Self {
        self.symlink = Some(symlink);
        self
    }

    pub fn use_entries_opt(&mut self, e_opt: EntriesOpt) -> &mut Self {
        self.e_opt = Some(e_opt);
        self
    }

    pub fn build(&self) -> WalkDir {
        WalkDir {
            path: {
                match pbuf_is_dir(&self.path) {
                    true => self.path.to_path_buf(),
                    false => panic!("Failed to build this WalkDir instance.\n{} doesn't exists or is not a directory.", self.path.display()),
                }
            },
            hidden: {
                match self.hidden {
                    Some(hidden) => hidden,
                    None => false,
                }
            },
            symlink: {
                 match self.symlink {
                    Some(symlink) => symlink,
                    None => false,
                }
            },
            e_opt: {
                match self.e_opt.clone() {
                    Some(e_opt) => e_opt,
                    None => EntriesOpt::default(),
                }
            }
        }
    }
}

pub fn is_hidden(s: &String) -> bool {
    s.starts_with(".")
}

impl WalkDir {

    pub fn scan(&self) -> Option<Entries> {
        let mut entries = Entries::new(&self.path);
        match self.path.read_dir() {
            Ok(dir_entries) => {
                for dir_entry in dir_entries {
                    if let Ok(entry) = dir_entry {
                       if let Ok(mdata) = entry.metadata() {
                            let mut go_push = true;
                            let name = entry.file_name().into_string().unwrap();

                            /* check user options */
                            if name.starts_with(".") && !self.hidden {
                                go_push = false;
                            }
                            else
                            if mdata.file_type().is_symlink() && !self.symlink {
                                go_push = false;
                            } 
                            
                            if go_push {
                                entries.push(Entry::new(name, mdata, &self.e_opt));
                            }
                       }
                    }
                }
                Some(entries)
            },
            Err(e) => {
                println!("Error: when scanning {} directory.", self.path.display());
                println!("-> {}", e);
                None
            }
        }
    }
}