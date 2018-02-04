//use walkdir::DirEntry;
//use walker::Walker;
use std::path::PathBuf;
//use rayon::prelude::*;
use chrono::prelude::*;
use std::ffi::{OsStr, OsString};
use std::fs::DirEntry;
//use config::Config;
use config::EntriesOpt;
//use std::fs::FileType;

//use std::thread;

use super::{STR_IBYTES, SHORT_STR_IBYTES, STR_BYTES, SHORT_STR_BYTES};

#[derive(Serialize, Deserialize)]
pub struct Entry<'a> {
    name: &'a str,
    size: Option<u64>,
    human_size: Option<f64>,
    long_unit_size: Option<&'a str>,
    short_unit_size: Option<&'a str>,
    timestamp: Option<i64>,
    datetime: Option<String>,
    directory: Option<bool>,
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

    pub fn new(dir_e: DirEntry, opt: &EntriesOpt) -> Self {

        let mut e = Entry {
            name: {
                let fname: &'a OsString = &dir_e.file_name();
                let possible_str_fname = fname.to_str();
                let lfname: &'a str = possible_str_fname.unwrap();
                lfname
                //dir_e.file_name().to_str().unwrap()
                //fname
            },
            size: None,
            human_size: None,
            long_unit_size: None,
            short_unit_size: None,
            timestamp: None,
            datetime: None,
            directory: None,
            elements: 1,
        };

        if let Ok(metadata) = dir_e.metadata() {

            if let Ok(system_time) = metadata.modified() {
                let chrono_datetime: DateTime<Local> = system_time.into();
                e.timestamp = Some(chrono_datetime.timestamp());                
                e.datetime = Some(chrono_datetime.format(&opt.datetime_format).to_string());
            }

            let mut index = 0usize;

            match metadata.is_dir() {
                true => {
                    e.directory = Some(true);
                    e.size = Some(0u64);
                    e.human_size = Some(0f64);
                },
                false => {
                    e.directory = Some(false);
                    e.size = Some(metadata.len());

                    /* Compute human readable size */
                    let divider: f64;
                    match opt.unit_size {
                        true => divider = 1024.0f64,
                        false => divider = 1000.0f64,
                    }
                    
                    let mut human_size = e.size.unwrap() as f64;
                    while human_size >= divider {
                        human_size /= divider;
                        index += 1;
                    }
                    
                    /* Truncate result to a certain float precision */
                    let size_string = human_size.to_string();
                    if let Some(dot_index) = size_string.as_str().find(".") {
                        let (size_str, _) = size_string.split_at(dot_index + 1 + opt.float_precision);
                        human_size = size_str.to_string().parse().unwrap();
                    }                   

                    e.human_size = Some(human_size);
                }
            }
            
            match opt.unit_size {
                true => {
                    e.long_unit_size = Some(STR_IBYTES[index]);
                    e.short_unit_size = Some(SHORT_STR_IBYTES[index]);
                },
                false => {
                    e.long_unit_size = Some(STR_BYTES[index]);
                    e.short_unit_size = Some(SHORT_STR_BYTES[index]);
                }
            }
        }

        return e
    }

    pub fn is_dir(&self) -> Option<bool> {
        self.directory
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> u64 {
        self.size.unwrap_or(0u64)
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
        match e.is_dir() {
            Some(is_dir) => {
                self.total_size += e.size();
                self.total_elts += 1;
                match is_dir {
                    true => self.directories.push( e ),
                    false => self.files.push( e ),
                }
            },
            None => {},
        }
    }

    /// Total elements
    pub fn telts(&mut self) -> u64 {
        let mut delts = 0u64;
        for dir in &self.directories {
            delts += dir.elements;
        }
        self.total_elts = delts + self.files.len() as u64;
        self.total_elts
    }

    /// Total size (bytes)
    pub fn tsize(&mut self) -> u64 {
        let mut dsize = 0u64;
        for dir in &self.directories {
            if let Some(size) = dir.size {
                dsize += size;
            }
        }
       
        let mut fsize = 0u64;
        for file in &self.files {
            if let Some(size) = file.size {
                fsize += size;
            }
        }

        self.total_size = dsize + fsize;
        self.total_size
    }

    pub fn dirs(&self) -> &Vec<Entry> {
        &self.directories
    }

    pub fn files(&self) -> &Vec<Entry> {
        &self.files
    }

    /*
    /// Process `deep_run` for each directory
    pub fn process_deep_run(&mut self, cfg: &Config) {

        self.directories.par_iter_mut()
                        .for_each(|dir|{
                            let walker = Walker::new(&dir.path, cfg);
                            let (dsize, delts) = walker.deep_run();
                            dir.size = dsize;
                            dir.elements += delts; /* Directory count as one element itself. Initialized to 1 in constructor. */
                        });

        /* Update total size & total elements */
        self.size = self.tsize();
        self.elements = self.telts();
    }

    pub fn toggle_prefix(self, old_prefix: &PathBuf, new_prefix: &PathBuf) {
        /*
        for mut dir in self.directories.into_iter() {
            dir.path = dir.path.strip_prefix(old_prefix).unwrap().to_path_buf();
            dir.path = new_prefix.join(dir.path);
        }
        
        for mut file in self.files.into_iter() {
            file.path = file.path.strip_prefix(old_prefix).unwrap().to_path_buf();
            file.path = new_prefix.join(file.path);
        }
        */
    }
    */
}