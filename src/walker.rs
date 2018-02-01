use std::path::Path;
use walkdir::WalkDir;
use filesystem::{is_hidden, is_symlink};
use filesystem::entries::*;

pub struct Walker<'a> {
    pathname: &'a Path,
    hidden: bool,
    symlink: bool,
}

impl<'a> Walker<'a> {
    
    pub fn new(pathname: &'a Path, hidden: bool, symlink: bool) -> Walker<'a> {
        Walker { 
            pathname,
            hidden,
            symlink,
        }
    }
    
    /// If min_depth == max_depth, option follows links become useless.
    /// Instead, use the entry.file_type().is_symlink() predicate.
    pub fn run(&self) -> Entries {
        let walker = WalkDir::new(self.pathname)
                    .min_depth(1)
                    .max_depth(1)
                    .into_iter()
                    .filter_entry(|e| (!is_hidden(e) | self.hidden) && (!is_symlink(e) | self.symlink) );

        let mut entries = Entries::new();

        for dir_entry in walker {
            if let Ok(dir_entry) = dir_entry {
                entries.push( Entry::new(&dir_entry) );
            }
        }

        entries
    }

    pub fn deep_run(&self) -> (u64, u64) {

        let walker = WalkDir::new(self.pathname)
                            .min_depth(1)
                            .follow_links(self.symlink)
                            .into_iter()
                            .filter_entry(|e| (!is_hidden(e) | self.hidden) );

        let mut size = 0u64;
        let mut elts = 0u64;

        for dir_entry in walker {
            if let Ok(dir_entry) = dir_entry {
                if dir_entry.file_type().is_file() {
                    if let Ok(metadata) = dir_entry.metadata() {
                        size += metadata.len();
                    }
                }
                elts += 1;
            }
        }

        (size, elts)
    }

}