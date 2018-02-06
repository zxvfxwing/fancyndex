use std::path::PathBuf;
use chrono::prelude::*;

use std::fs::Metadata;

use rayon::prelude::*;

use config::EntriesOpt;
use filesystem::{pbuf_str, pbuf_parent, pbuf_vstring};

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    name: String,
    size: u64,
    time: i64,
    directory: bool,
    elements: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Entries {
    root: PathBuf,
    parent: PathBuf,
    root_vec: Vec<String>,
    total_size: u64,
    total_elts: u64,
    directories: Vec<Entry>,
    files: Vec<Entry>,
    opt: EntriesOpt,
}

pub struct EntriesBuilder {
    root: PathBuf,
    opt: Option<EntriesOpt>,
}

impl Entry {

    pub fn new(name: String, metadata: Metadata) -> Self {
        let size: u64;
        let directory = metadata.is_dir();

        match directory {
            true =>  size = 0u64,
            false => size = metadata.len(),
        }

        /* Timestamp */
        let mut time = 0i64;
        if let Ok(system_time) = metadata.modified() {
            let chrono_datetime: DateTime<Local> = system_time.into();
            time = chrono_datetime.timestamp();           
        }
   
        Entry {
            name,
            size,
            time,
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

    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    pub fn set_elts(&mut self, elts: u64) {
        self.elements = elts;
    }
}

impl Entries {

    /// Add a new Entry
    pub fn push(&mut self, e: Entry) {
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

    pub fn dirs(&mut self) -> &mut Vec<Entry> {
        &mut self.directories
    }

    pub fn files(&mut self) -> &mut Vec<Entry> {
        &mut self.files
    }

    /// For security reason, don't show the system absolute path on the internet.
    /// Toggle prefix with the "URL version" of absolute path (GET routes).
    pub fn toggle_prefix(&mut self, old_prefix: &PathBuf, new_prefix: &PathBuf) {
        self.root = self.root.strip_prefix(old_prefix).unwrap().to_path_buf();
        /* a e s t h e t i c */
        if pbuf_str(&self.root) != "" {
            self.root = new_prefix.join(&self.root);
        }
        else {
            self.root = new_prefix.to_path_buf();
        }

        self.parent = pbuf_parent(&self.root);
        self.root_vec = pbuf_vstring(&self.root);
    }
}

impl EntriesBuilder {
    pub fn new(root: PathBuf) -> Self {
        EntriesBuilder {
            root,
            opt: None,
        }
    }

    pub fn use_entries_opt(&mut self, opt: EntriesOpt) -> &mut Self {
        self.opt = Some(opt);
        self
    }

    pub fn build(&self) -> Entries {
        Entries {
            root: self.root.to_path_buf(),
            parent: pbuf_parent(&self.root),

            /*
            * Since I failed to use Tera template built-in filter `split` due to Rocket version:
            * Here we are, sending a Vec<String>, and of course 
            * I can't even handle lifetime with &'str cause of compile errors everwhere.
            */
            root_vec: pbuf_vstring(&self.root),
            total_size: 0u64,
            total_elts: 0u64,
            directories: Vec::new(),
            files: Vec::new(),
            opt: {
                match self.opt.clone() {
                    Some(opt) => opt,
                    None => EntriesOpt::default(),
                }
            },
        }
    }
}