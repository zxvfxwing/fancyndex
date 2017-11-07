/* Module directory */

/* Wrong choice */

use std::path::PathBuf;
use utils::error;

pub struct Directory {
    path: PathBuf,
    depth: i64,
    max_depth: i64,
    do_hidden: bool,
    do_symlink: bool,
    directories: Vec<Directory>,
    files: Vec<PathBuf>
}

impl Directory {

    /*
    * Create an init directory, where depth == 0
    * Should be used only for the first dir, not his entries /!\
    */
    pub fn new_one(p: &PathBuf) -> Directory {

        if !p.is_dir() {
            error::err_msg("PathBuf passed as parameter isn't a directory.");
            error::err_msg("You can't make an instance of Directory struct with this PathBuf !");
            error::exit(false);
        }

        Directory {
            path: p.to_path_buf(),
            depth: 0i64,
            max_depth: -1i64,
            do_hidden: false,
            do_symlink: false,
            directories: Vec::new(),
            files: Vec::new(),
        }
    }

    /*
    * Create a directory and set his depth
    */
    pub fn new(p: &PathBuf, depth: i64) -> Directory {
        Directory::new_one(p).deepness(depth)
    }

    /*
    * Set the depth of the actual directory
    */
    pub fn deepness(mut self, depth: i64) -> Directory {
        self.depth = depth;
        self
    }

    /*
    * Set the max depth wanted to reach when walking throught directories
    *
    * max_depth < 0 == no limits
    * 0, is the actual directory, without walking in
    * 1, directory's children
    * [...]
    */
    pub fn max_depth(mut self, max_depth: i64) -> Directory {
        self.max_depth = max_depth;
        self
    }

    pub fn follow_symlink(mut self, follow: bool) -> Directory {
        self.do_symlink = follow;
        self
    }

    pub fn read_hiddden(mut self, read: bool) -> Directory {
        self.do_hidden = read;
        self
    }

    fn add_entry(&mut self, entry: &PathBuf) {
        if entry.is_dir() {
            let mut dir = Directory::new(&entry, self.depth + 1);
            dir.deep_run();
            self.directories.push(dir);
        }

        else
        if entry.is_file() {
            self.files.push(entry.to_path_buf());
        }

        else
        if self.do_symlink {
            if let Ok(link) = entry.read_link() {
                self.add_entry(&link);
            }
        }
    }

    pub fn deep_run(&mut self) {
        if self.max_depth < 0 || self.depth <= self.max_depth {
            if let Ok(entries) = self.path.read_dir() {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let entry_path = entry.path();
                        let is_hidden = super::is_hidden(&entry_path);

                        if is_hidden && !self.do_hidden { continue; }
                        //println!("{}", entry_path.display());
                        self.add_entry(&entry_path);
                    }
                }
            }
        }
    }

}
