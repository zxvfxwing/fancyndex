use std::path::PathBuf;
use filesystem::file::File;

#[derive(Serialize)]
pub struct Directory {
    compo: Vec<String>,
    name: String,
    size: u64,
    hsize: f64,
    unit: String,
    short_unit: String,
    timestamp: i64,
    datetime: String,
    elements: u64,
    directories: Vec<Directory>,
    files: Vec<File>,
}

impl Directory {

    pub fn new(p: &PathBuf, size: u64, elts: u64, mode: bool) -> Directory {
        let hsize = super::get_human_size(size, mode);

        let cdir_name = super::get_filename(&super::get_parent_cdir());
        let home_str = "home";

        Directory {

            compo: {
                let mut compo: Vec<String> = Vec::new();
                let iter = p.iter();
                let mut do_add = false;
                //let mut full_str = String::new();

                for i in iter {
                    let mut iter_str = i.to_str().unwrap();

                    if iter_str == cdir_name {
                        do_add = true;
                        iter_str = home_str;
                    }

                    if do_add {
                        //full_str += "/";
                        //full_str += &iter_str;
                        //full_str = format!("{}/{}", full_str, iter_str);
                        compo.push(iter_str.to_string());
                    }
                }

                compo
            },
            name: {
                let dir_name = super::get_filename(p);

                if dir_name != cdir_name {
                    dir_name
                }
                else{
                    home_str.to_string()
                }
            },
            size: size,
            hsize: hsize.0,
            unit: hsize.1,
            short_unit: hsize.2,
            timestamp: super::get_timestamp(p),
            datetime: super::get_datetime(p),
            elements: elts,
            directories: Vec::new(),
            files: Vec::new(),
        }
    }

    pub fn add_dirs(&mut self, new: Vec<Directory>) {
        self.directories = new;
    }

    pub fn add_files(&mut self, new: Vec<File>) {
        self.files = new;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_size(&self) -> u64  {
        self.size
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn sort_by_size_ascending(&mut self) {
        self.directories.sort_unstable_by(|a, b| a.get_size().cmp(&b.get_size()));
        self.files.sort_unstable_by(|a, b| a.get_size().cmp(&b.get_size()));
    }

    pub fn sort_by_size_descending(&mut self) {
        self.directories.sort_unstable_by(|a, b| b.get_size().cmp(&a.get_size()));
        self.files.sort_unstable_by(|a, b| b.get_size().cmp(&a.get_size()));
    }

    pub fn sort_by_time_ascending(&mut self) {
        self.directories.sort_unstable_by(|a, b| a.get_timestamp().cmp(&b.get_timestamp()));
        self.files.sort_unstable_by(|a, b| a.get_timestamp().cmp(&b.get_timestamp()));
    }

    pub fn sort_by_time_descending(&mut self) {
        self.directories.sort_unstable_by(|a, b| b.get_timestamp().cmp(&a.get_timestamp()));
        self.files.sort_unstable_by(|a, b| b.get_timestamp().cmp(&a.get_timestamp()));
    }

    pub fn sort_by_name_ascending(&mut self) {
        self.directories.sort_unstable_by(|a, b| a.get_name().to_lowercase().cmp(&b.get_name().to_lowercase()));
        self.files.sort_unstable_by(|a, b| a.get_name().to_lowercase().cmp(&b.get_name().to_lowercase()));
    }

    pub fn sort_by_name_descending(&mut self) {
        self.directories.sort_unstable_by(|a, b| b.get_name().to_lowercase().cmp(&a.get_name().to_lowercase()));
        self.files.sort_unstable_by(|a, b| b.get_name().to_lowercase().cmp(&a.get_name().to_lowercase()));
    }
}
