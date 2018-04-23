use std::path::PathBuf;
use walkdir::{
    WalkDir, 
    DirEntry
};

use filesystem::{
    Directory,
    pbuf_fname
};

#[derive(Serialize, Deserialize)]
pub struct Scanner {
    path: PathBuf,
    hidden: bool,
    symlink: bool,
}

impl Scanner {
    pub fn new(p: &PathBuf) -> Self {
        Scanner {
            path: p.to_owned(),
            hidden: false,
            symlink: false, 
        }
    }

    pub fn do_hidden(mut self, h: bool) -> Self {
        self.hidden = h;
        self
    }

    pub fn do_symlink(mut self, s: bool) -> Self {
        self.symlink = s;
        self
    }

    pub fn is_hidden(entry: &DirEntry) -> bool {
        entry.file_name()
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    }

    pub fn entries(&self) -> Directory {
        let mut dir = Directory::new(&self.path);
        
        let mut walker =
            WalkDir::new(&self.path)
                .min_depth(1)
                .max_depth(1)
                .follow_links(self.symlink)
                .into_iter();
        
        loop {
            let entry =
                match walker.next() {
                    None => break,
                    Some(Err(_)) => continue,
                    Some(Ok(entry)) => entry,
                };
            
            if Scanner::is_hidden(&entry) && !self.hidden {
                if entry.file_type().is_dir() {
                    walker.skip_current_dir();
                }
            }
            else {
                dir.push_entry(&entry);
            }
        }

        dir
    }

    pub fn deep_run(&self) -> Directory {
        let mut bytes = 0u64;
        let mut elements = 0u64;

        let mut walker =
            WalkDir::new(&self.path)
                .min_depth(1)
                .follow_links(self.symlink)
                .into_iter();
        
        loop {
            let entry =
                match walker.next() {
                    None => break,
                    Some(Err(_)) => continue,
                    Some(Ok(entry)) => entry,
                };
            
            if Scanner::is_hidden(&entry) && !self.hidden {
                if entry.file_type().is_dir() {
                    walker.skip_current_dir();
                }
            }
            else {
                if entry.file_type().is_file() {
                    elements += 1;
                    if let Ok(metadata) = entry.metadata() {
                        bytes += metadata.len();
                    }
                }
            }
        }

        Directory {
            name: pbuf_fname(&self.path),
            path: self.path.to_owned(),
            bytes,
            elements,
            directories: Vec::new(),
            files: Vec::new(),
        }
    }
}