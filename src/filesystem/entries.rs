use std::path::PathBuf;
use chrono::prelude::*;
use std::ffi::{OsStr, OsString};
use std::fs::DirEntry;

use std::fs::Metadata;

use rayon::prelude::*;

use config::EntriesOpt;
use filesystem::pbuf_str;
use super::{
    STR_BYTES,
    STR_IBYTES,
    SHORT_STR_BYTES, 
    SHORT_STR_IBYTES,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry<'a> {
    name: String,
    size: u64,
    human_size: f64,
    long_unit_size: &'a str,
    short_unit_size: &'a str,
    timestamp: i64,
    datetime: String,
    directory: bool,
    elements: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Entries<'a> {
    root: PathBuf,
    total_size: u64,
    total_elts: u64,
    #[serde(borrow)]
    directories: Vec<Entry<'a>>,
    #[serde(borrow)]
    files: Vec<Entry<'a>>,
}

impl<'a> Entry<'a> {

    pub fn new(name: String, metadata: Metadata, opt: &EntriesOpt) -> Self {        
        let directory: bool = metadata.is_dir();
        let size: u64 = metadata.len();

        let mut power_index = 0usize;
        let mut human_size = 0f64;

        /* If it's a file, compute its human readable size */
        if !directory {
            let divider: f64;
            match opt.unit_size {
                true => divider = 1024.0f64,
                false => divider = 1000.0f64,
            }
            
            human_size = size as f64;
            while human_size >= divider {
                human_size /= divider;
                power_index += 1;
            }   
            
            /* Truncate result to a certain float precision */
            let size_string = human_size.to_string();
            if let Some(dot_index) = size_string.as_str().find(".") {
                let (size_str, _) = size_string.as_str().split_at(dot_index + 1 + opt.float_precision);
                human_size = size_str.parse().unwrap();
            }                 
        }

        /* Unit Size */
        let long_unit_size: &str;
        let short_unit_size: &str;

        match opt.unit_size {
            true => {
                long_unit_size = STR_IBYTES[power_index];
                short_unit_size = SHORT_STR_IBYTES[power_index];
            },
            false => {
                long_unit_size = STR_BYTES[power_index];
                short_unit_size = SHORT_STR_BYTES[power_index];
            }
        }

        /* Timestamp & Datetime */
        let mut timestamp = 0i64;
        let mut datetime = "1970-01-01 00:00:00".to_string();

        if let Ok(system_time) = metadata.modified() {
            let chrono_datetime: DateTime<Local> = system_time.into();
            timestamp = chrono_datetime.timestamp();           
            datetime = chrono_datetime.format(&opt.datetime_format).to_string();
        }
   
        Entry {
            name,
            size,
            human_size,
            long_unit_size,
            short_unit_size,
            timestamp,
            datetime,
            directory,
            elements: 1,
        }
    }

    pub fn is_dir(&self) -> bool {
        self.directory
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn set_size(&mut self, size: u64, opt: &EntriesOpt) {
        self.size = size;

        let mut power_index = 0usize;
        let mut human_size = 0f64;

        let divider: f64;
        match opt.unit_size {
            true => divider = 1024.0f64,
            false => divider = 1000.0f64,
        }
        
        human_size = size as f64;
        while human_size >= divider {
            human_size /= divider;
            power_index += 1;
        }   
        
        /* Truncate result to a certain float precision */
        let size_string = human_size.to_string();
        if let Some(dot_index) = size_string.as_str().find(".") {
            let (size_str, _) = size_string.as_str().split_at(dot_index + 1 + opt.float_precision);
            human_size = size_str.parse().unwrap();
        }

        self.human_size = human_size;

        match opt.unit_size {
            true => {
                self.long_unit_size = STR_IBYTES[power_index];
                self.short_unit_size = SHORT_STR_IBYTES[power_index];
            },
            false => {
                self.long_unit_size = STR_BYTES[power_index];
                self.short_unit_size = SHORT_STR_BYTES[power_index];
            }
        }
    }

    pub fn set_elts(&mut self, elts: u64) {
        self.elements = elts;
    }
}

impl<'a> Entries<'a> {
    pub fn new(root: &PathBuf) -> Self {
        Entries {
            root: root.to_path_buf(),
            total_size: 0u64,
            total_elts: 0u64,
            directories: Vec::new(),
            files: Vec::new(),
        }
    }

    /// Add a new Entry
    pub fn push(&mut self, e: Entry<'a>) {
        self.total_size += e.size();
        self.total_elts += 1;
        match e.is_dir() {           
            true => self.directories.push(e),
            false => self.files.push(e),
        }
    }

    /// Total elements
    pub fn telts(&mut self) -> u64 {
        let delts: u64 = self.directories.par_iter_mut()
                                         .map(|dir| dir.elements)
                                         .sum();

        self.total_elts = delts + self.files.len() as u64;
        self.total_elts
    }

    /// Total size (bytes)
    pub fn tsize(&mut self) -> u64 {
        let dsize: u64 = self.directories.par_iter_mut()
                                         .map(|dir| dir.size)
                                         .sum();

        let fsize: u64 = self.files.par_iter_mut()
                                   .map(|file| file.size)
                                   .sum();

        self.total_size = dsize + fsize;
        self.total_size
    }

    pub fn dirs(&mut self) -> &mut Vec<Entry<'a>> {
        &mut self.directories
    }

    pub fn files(&mut self) -> &mut Vec<Entry<'a>> {
        &mut self.files
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn set_dirs(&mut self, dirs: Vec<Entry<'a>>) {
        self.directories = dirs.to_vec();
    }

    pub fn add_to_tsize(&mut self, size: u64) {
        self.total_size += size;
    }

    pub fn add_to_telts(&mut self, elts: u64) {
        self.total_elts += elts;
    }

    /// For security reason, don't show the system absolute path on the internet.
    /// Toggle prefix with the "URL version" of absolute path (GET routes).
    pub fn toggle_root_prefix(&mut self, old_prefix: &PathBuf, new_prefix: &PathBuf) {
        self.root = self.root.strip_prefix(old_prefix).unwrap().to_path_buf();
        /* a e s t h e t i c */
        if pbuf_str(&self.root) != "" {
            self.root = new_prefix.join(&self.root);
        }
        else {
            self.root = new_prefix.to_path_buf();
        }
    }
}