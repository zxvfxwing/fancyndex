use std::path::Path;
use walkdir::WalkDir;
use filesystem::{dir_e_is_hidden, dir_e_is_symlink};
use filesystem::entries::{Entries, Entry};

pub struct Walker<'a> {
    path: &'a Path,
    hidden: bool,
    symlink: bool,
}

impl<'a> Walker<'a> {
    
    pub fn new(path: &'a Path, hidden: bool, symlink: bool) -> Walker<'a> {
        Walker { 
            path,
            hidden,
            symlink,
        }
    }
    
    /// If min_depth == max_depth, option follows links become useless.
    /// Instead, use the entry.file_type().is_symlink() predicate.
    pub fn run(&self) -> Entries {
        let walkdir = WalkDir::new(self.path)
                    .min_depth(1)
                    .max_depth(1)
                    .into_iter()
                    .filter_entry(|e| (!dir_e_is_hidden(e) | self.hidden) && (!dir_e_is_symlink(e) | self.symlink) );

        let mut entries = Entries::new();
        walkdir.for_each(|e| {
            entries.push( Entry::new(&e.unwrap()) );
        });
        entries
    }

    pub fn deep_run(&self) -> (u64, u64) {

        let walkdir = WalkDir::new(self.path)
                            .min_depth(1)
                            .follow_links(self.symlink)
                            .into_iter()
                            .filter_entry(|e| (!dir_e_is_hidden(e) | self.hidden) );

        let mut size = 0u64;
        let mut elts = 0u64;

        walkdir.for_each(|e| {
            if let Ok(e) = e {
                if e.file_type().is_file() {
                    size += e.metadata().unwrap().len();
                }
                elts += 1;
            }
        });

        (size, elts)
    }

}