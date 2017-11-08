/*
* Filesystem Module
*/

pub mod directory;

/* Use */
/* STD Lib */
use std::env;
use std::process;
use std::path::PathBuf;
use std::time::SystemTime;

/* WalDir extern crate */
use walkdir::DirEntry;

/* Chrono extern crate */
use chrono::prelude::*;

/* own modules utils::error */
use utils::error;



/* Constant */
static BYTES: &'static [&str] = &[
    "Byte(s)",
    "KiloByte(s)",
    "MegaByte(s)",
    "GigaByte(s)",
    "TeraByte(s)",
    "PetaByte(s)",
    "ExaByte(s)",
    "ZettaByte(s)",
    "YottaByte(s)"
];

static A_BYTES: &'static [&str] = &[
    "B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"
];

static IBYTES: &'static [&str] = &[
    "Byte(s)",
    "KibiByte(s)",
    "MebiByte(s)",
    "GibiByte(s)",
    "TebiByte(s)",
    "PebiByte(s)",
    "ExbiByte(s)",
    "ZebiByte(s)",
    "YobiByte(s)"
];

static A_IBYTES: &'static [&str] = &[
    "B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"
];

/*
* Get current directory's std::path::PathBuf
*
* Possible enhancement (works also for the two next functions):
* -> Don't abort program when not finding dir
*   -> ErrMessage, Try/Catch, Throw exception.
*
* return std::path::PathBuf
*/
pub fn get_current_dir() -> PathBuf {
    match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            error::err_msg(&e.to_string()[..]);
            error::err_msg("Error: Failed when trying to access current directory.");
            error::err_msg("Please check permissions !");
            error::exit(false);
        }
    }
}

/*
* Get directory's parent, return std::path::PathBuf
*/
pub fn get_parent_dir(p: &PathBuf) -> PathBuf {
    match p.parent() {
        Some(parent) => parent.to_path_buf(),
        None => {
            error::err_msg(
                format!(
                    "We can't access to the parent directory of {} .", p.display()
                ).as_str()
            );
            error::err_msg("Please check permissions !");
            error::exit(false);
        }
    }
}

/* Get current directory's parent, return std::path::PathBuf */
pub fn get_parent_cdir() -> PathBuf {
    get_parent_dir(&get_current_dir())
}

pub fn get_filename(p: &PathBuf) -> String {
    match p.file_name() {
        None => panic!("No file_name found !"),
        Some(filename) => match filename.to_str() {
            None => panic!("No str in file_name !"),
            Some(fname_str) => fname_str.to_string()
        }
    }
}

/*
* Get systemTime
*/
pub fn dir_entry_systemtime(entry: &DirEntry) -> SystemTime {
    match entry.metadata() {
        Ok(metadata) => {
            match metadata.modified() {
                Ok(time) => time,
                Err(_) => SystemTime::now()
            }
        }
        Err(_) => SystemTime::now()
    }
}

pub fn dir_entry_datetime(entry: &DirEntry) -> String {
    let datetime: DateTime<Local> = dir_entry_systemtime(entry).into();
    datetime.format("%Y-%m-%d %T").to_string()
}

pub fn dir_entry_timestamp(entry: &DirEntry) -> i64 {
    let datetime: DateTime<Local> = dir_entry_systemtime(entry).into();
    datetime.timestamp()
}

pub fn is_hidden(path: &PathBuf) -> bool {
    get_filename(path).starts_with(".")
}

pub fn dir_entry_size(entry: &DirEntry) -> u64 {
    match entry.metadata() {
        Ok(metadata) => {
            metadata.len()
        }
        Err(_) => 0u64
    }
}

pub fn dir_entry_type(entry: &DirEntry) -> bool {
    entry.file_type().is_file()
}

/*
#[derive(Serialize)]
pub struct Entry {
    name: String,
    size: u64,
    hsize: f64,
    unit: String,
    acro_unit: String,
    timestamp: i64,
    datetime: String,
    ftype: bool,
    elements: u64
}

impl Entry {

    pub fn binary_prefix(&mut self) {
        let mut hsize: f64 = self.size as f64;
        let mut power = 0usize;
        let u = 1024.0f64;

        while hsize >= u {
            hsize /= u;
            power+=1;
        }

        self.hsize = hsize;
        if power > 0 {
            self.unit = IBYTES[power].to_string();
            self.acro_unit = A_IBYTES[power].to_string();
        }
    }

    pub fn decimal_prefix(&mut self) {
        let mut hsize: f64 = self.size as f64;
        let mut power = 0usize;
        let u = 1000.0f64;

        while hsize >= u {
            hsize /= u;
            power+=1;

        }

        self.hsize = hsize;
        if power > 0 {
            self.unit = IBYTES[power].to_string();
            self.acro_unit = A_IBYTES[power].to_string();
        }
    }

    pub fn empty() -> Entry {
        Entry {
            name: "__".to_string(),
            size: 0u64,
            hsize: 0.0f64,
            unit: BYTES[0].to_string(),
            acro_unit: A_BYTES[0].to_string(),
            timestamp: 0i64,
            datetime: "../../..".to_string(),
            ftype: true,
            elements: 0
        }
    }

    pub fn new(entry: &DirEntry, mode: bool) -> Entry {

        let mut e = Entry {
            name: dir_entry_fname(entry),
            size: dir_entry_size(entry),
            hsize: 0f64,
            unit: BYTES[0].to_string(),
            acro_unit: A_BYTES[0].to_string(),
            timestamp: dir_entry_timestamp(entry),
            datetime: dir_entry_datetime(entry),
            ftype: dir_entry_type(entry),
            elements: 0u64,
        };

        /* Default, using Binary Prefix */
        if mode { e.binary_prefix(); }
        else { e.decimal_prefix(); }
        return e
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn set_size(&mut self, size: u64, mode: bool) {
        self.size = size;
        if mode { self.binary_prefix(); }
        else { self.decimal_prefix(); }
    }

    pub fn set_elements(&mut self, elements: u64) {
        self.elements = elements;
    }
}
*/
