use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use filesystem::is_hidden;

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
    
    pub fn run(&self) -> Vec<PathBuf> {
        let walker = WalkDir::new(self.pathname)
                    .min_depth(1)
                    .max_depth(1)
                    .follow_links(self.symlink)
                    .into_iter()
                    .filter_entry(|e| !is_hidden(e) | self.hidden);

        let mut vec = Vec::new();

        for entry in walker {
            if let Ok(entry) = entry {
                println!("{}", entry.path().display());
                vec.push( entry.path().to_path_buf() );
            }
        }

        vec
    }

    pub fn deep_run(&self) -> (u64, u64) {
        let walker = WalkDir::new(self.pathname)
                            .min_depth(1)
                            .follow_links(self.symlink)
                            .into_iter()
                            .filter_entry(|e| !is_hidden(e) | self.hidden);

        let mut size = 0u64;
        let mut elts = 0u64;

        for entry in walker {
            if let Ok(entry) = entry {
                if entry.file_type().is_file() {
                    if let Ok(metadata) = entry.metadata() {
                        size += metadata.len();
                    }
                }
                elts += 1;
            }
        }

        (size, elts)
    }

}