use std::path::PathBuf;
use filesystem::file::File;

#[derive(Serialize)]
pub struct Directory {
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

        Directory {
            name: super::get_filename(p),
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
        self.directories.sort_unstable_by(|b, a| a.get_size().cmp(&b.get_size()));
        self.files.sort_unstable_by(|b, a| a.get_size().cmp(&b.get_size()));
    }

    pub fn sort_by_time_ascending(&mut self) {
        self.directories.sort_unstable_by(|a, b| a.get_timestamp().cmp(&b.get_timestamp()));
        self.files.sort_unstable_by(|a, b| a.get_timestamp().cmp(&b.get_timestamp()));
    }

    pub fn sort_by_time_descending(&mut self) {
        self.directories.sort_unstable_by(|b, a| a.get_timestamp().cmp(&b.get_timestamp()));
        self.files.sort_unstable_by(|b, a| a.get_timestamp().cmp(&b.get_timestamp()));
    }

    pub fn sort_by_name_ascending(&mut self) {
        self.directories.sort_unstable_by(|a, b| a.get_name().to_lowercase().cmp(&b.get_name().to_lowercase()));
        self.files.sort_unstable_by(|a, b| a.get_name().to_lowercase().cmp(&b.get_name().to_lowercase()));
    }

    pub fn sort_by_name_descending(&mut self) {
        self.directories.sort_unstable_by(|b, a| a.get_name().to_lowercase().cmp(&b.get_name().to_lowercase()));
        self.files.sort_unstable_by(|b, a| a.get_name().to_lowercase().cmp(&b.get_name().to_lowercase()));
    }
}
