use walkdir::DirEntry;
use walker::Walker;
use std::path::PathBuf;
use rayon::prelude::*;
use chrono::prelude::*;
use std::ffi::OsString;

use config::EntriesOpt;
use super::{STR_IBYTES, SHORT_STR_IBYTES, STR_BYTES, SHORT_STR_BYTES};

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub path: PathBuf,
    pub name: OsString,
    pub size: Option<u64>,
    pub human_size: Option<f64>,
    pub long_unit_size: Option<String>,
    pub short_unit_size: Option<String>,
    pub timestamp: Option<i64>,
    pub datetime: Option<String>,
    pub elements: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Entries {
    pub size: u64,
    pub elements: u64,
    pub directories: Vec<Entry>,
    pub files: Vec<Entry>,
}

impl Entry {

    pub fn new(dir_e: &DirEntry) -> Entry {
        Entry {
            path: dir_e.path().to_path_buf(),
            name: dir_e.file_name().to_os_string(),
            size: None,
            human_size: None,
            long_unit_size: None,
            short_unit_size: None,
            timestamp: None,
            datetime: None,
            elements: 1,
        }
    }

    pub fn is_file(&self) -> bool {
        match self.path.metadata() {
            Ok(metadata) => metadata.file_type().is_file(),
            Err(_) => false,
         }
    }

    pub fn seek_metadata(&mut self, opt: &EntriesOpt) {
        if let Ok(metadata) = self.path.metadata() {
            
            if let Ok(system_time) = metadata.modified() {
                let chrono_datetime: DateTime<Local> = system_time.into();
                self.timestamp = Some(chrono_datetime.timestamp());                
                self.datetime = Some(chrono_datetime.format(&opt.datetime_format).to_string());
            }

            self.size = Some(metadata.len());

            /* Compute human readable size */
            let divider: f64;
            match opt.unit_size {
                true => divider = 1024.0f64,
                false => divider = 1000.0f64,
            }
            
            let mut index = 0usize;
            let mut human_size = self.size.unwrap() as f64;
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

            self.human_size = Some(human_size);

            match opt.unit_size {
                true => {
                    self.long_unit_size = Some( STR_IBYTES[index].to_string() );
                    self.short_unit_size = Some( SHORT_STR_IBYTES[index].to_string() );
                },
                false => {
                    self.long_unit_size = Some( STR_BYTES[index].to_string() );
                    self.short_unit_size = Some( SHORT_STR_BYTES[index].to_string() );
                }
            }
        }
    }
}

impl Entries {
    pub fn new() -> Entries {
        Entries{
            size: 0u64,
            elements: 0u64,
            directories: Vec::new(),
            files: Vec::new(),
        }
    }

    /// Add a new Entry
    /// Vector push depends of Entry type (file or not)
    pub fn push(&mut self, e: Entry) {
        if e.is_file() {
            self.files.push( e );
        }
        else {
            self.directories.push( e );
        }
        self.elements += 1;
    }

    /// Total elements
    pub fn telts(&self) -> u64 {
        let delts:u64 = self.directories.par_iter()
                                        .map(|dir| dir.elements)
                                        .sum();
        
        delts + self.files.len() as u64
    }

    /// Total size (bytes)
    pub fn tsize(&self) -> u64 {
        let dsize:u64 = self.directories.par_iter()
                                        .map(|dir| dir.size.unwrap_or(0u64))
                                        .sum();

        let fsize:u64 = self.files.par_iter()
                                  .map(|file| file.size.unwrap_or(0u64))
                                  .sum();
          
        dsize + fsize
    }

    pub fn fill_metadatas(mut self, opt: &EntriesOpt) -> Self {
        self.directories.par_iter_mut()
                        .for_each(|dir|{
                            dir.seek_metadata(opt);
                        });

        self.files.par_iter_mut()
                  .for_each(|file|{
                      file.seek_metadata(opt);
                  });
        
        self.size = self.tsize();
        
        self
    }

    /// Process `deep_run` for each directory
    pub fn process_deep_run(&mut self, hidden: bool, symlink: bool) {

        self.directories.par_iter_mut()
                        .for_each(|dir|{
                            let walker = Walker::new(&dir.path, hidden, symlink);
                            let (dsize, delts) = walker.deep_run();
                            dir.size = Some(dsize);
                            dir.elements += delts; /* Directory count as one element itself. Initialized to 1 in constructor. */
                        });

        /* Update total size & total elements */
        self.size = self.tsize();
        self.elements = self.telts();
    }

    pub fn toggle_prefix(mut self, old_prefix: &PathBuf, new_prefix: &PathBuf) -> Self {
        self.directories.par_iter_mut()
                        .for_each(|dir|{
                            dir.path = dir.path.strip_prefix(old_prefix)
                                               .unwrap()
                                               .to_path_buf();

                            dir.path = new_prefix.join(&dir.path);
                        });

        self.files.par_iter_mut()
                   .for_each(|file|{
                       file.path = file.path.strip_prefix(old_prefix)
                                            .unwrap()
                                            .to_path_buf();

                            file.path = new_prefix.join(&file.path);
                        });

        self
    }
}