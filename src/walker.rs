use std::path::Path;
use walkdir::WalkDir;
use filesystem::{dir_e_is_hidden, dir_e_is_symlink};
use filesystem::entries::{Entries, Entry};
use config::Config;

pub struct Walker<'a> {
    path: &'a Path,
    cfg: &'a Config,
}

impl<'a> Walker<'a> {
    
    pub fn new(path: &'a Path, cfg: &'a Config) -> Walker<'a> {
        Walker { 
            path,
            cfg,
        }
    }
    
    /// If min_depth == max_depth, option follows links become useless.
    /// Instead, use the entry.file_type().is_symlink() predicate.
    pub fn run(&self) -> Entries {
        let walkdir = WalkDir::new(self.path)
                    .min_depth(1)
                    .max_depth(1)
                    .into_iter()
                    .filter_entry(|e| {
                        (!dir_e_is_hidden(e) | self.cfg.walk_opt.hidden) && 
                        (!dir_e_is_symlink(e) | self.cfg.walk_opt.symlink) 
                    });

        let mut entries = Entries::new();
        for e in walkdir {
            if let Ok(e) = e {
                entries.push(Entry::new(&e, &self.cfg.entries_opt));
            }
        }

        entries.size = entries.tsize();
        entries
    }


    pub fn deep_run(&self) -> (u64, u64) {

        let walkdir = WalkDir::new(self.path)
                    .min_depth(1)
                    .follow_links(self.cfg.walk_opt.symlink)
                    .into_iter()
                    .filter_entry(|e| (!dir_e_is_hidden(e) | self.cfg.walk_opt.hidden) );    

        let mut size = 0u64;
        let mut elts = 0u64;

        for e in walkdir {
            if let Ok(e) = e {
                if e.file_type().is_file() {
                    if let Ok(md) = e.metadata() {
                        size += md.len();
                    }
                }
                elts += 1;
            }
        }

        (size, elts)
    }

}