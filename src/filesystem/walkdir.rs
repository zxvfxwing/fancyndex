use std::path::PathBuf;
use std::fs::ReadDir;
use std::io::Error;

use rayon::prelude::*;

use filesystem::entries::{EntriesBuilder, Entries, Entry};
use config::EntriesOpt;

use filesystem::{pbuf_is_dir, pbuf_str};

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

impl WalkDir {

    pub fn scan(&self) -> Option<Entries> {
        let mut entries = EntriesBuilder::new(self.path.to_path_buf())
                                        .use_entries_opt(self.e_opt.clone())
                                        .build();

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
                                /* Check symlink */
                                if mdata.file_type().is_symlink() {
                                    if let Ok(pbuf_link) = entry.path().read_link() {
                                        if let Ok(mdata) = pbuf_link.metadata() {
                                            entries.push(Entry::new(name, mdata));
                                        }
                                    }
                                }
                                else {
                                    entries.push(Entry::new(name, mdata));
                                }
                            }
                       }
                    }
                }
                Some(entries)
            },
            Err(e) => {
                println!("Error when scanning {} directory.", self.path.display());
                println!("-> {}", e);
                None
            }
        }
    }

    pub fn scan_entries(&self, e: &mut Entries) {
        e.dirs().par_iter_mut()
                .for_each(|dir|{
                    let(dsize, delts) = self.deep_run(self.path.join(dir.name()));
                    dir.set_size(dsize);
                    dir.set_elts(delts);
                });
        
        e.tsize();
        e.telts();
    }

    fn deep_run(&self, p: PathBuf) -> (u64, u64) {
        let mut tsize = 0u64;
        let mut telts = 0u64;

        if let Ok(dir_entries) = p.read_dir() {
            for dir_entry in dir_entries {
                if let Ok(entry) = dir_entry {
                    if let Ok(mdata) = entry.metadata() {
                        let mut count = true;
                        let name = entry.file_name().into_string().unwrap();

                        /* check user options */
                        if name.starts_with(".") && !self.hidden {
                            count = false;
                        }
                        else
                        if mdata.file_type().is_symlink() && !self.symlink {
                            count = false;
                        } 
                        
                        if count {
                            if mdata.file_type().is_dir() {
                                let (dsize, delts) = self.deep_run(p.join(name));
                                tsize += dsize;
                                telts += delts;
                            }
                            else {
                                tsize += mdata.len();
                                telts += 1;
                            }
                        }
                    }
                }
            }
        }

        (tsize, telts)
    }
}