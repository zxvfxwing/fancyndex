/* Use */
/* STD Lib */
use std::env;
use std::process;
use std::path::PathBuf;
use std::time::SystemTime;

/* Chrono extern crate */
use chrono::prelude::*;

/* WalkDir DirEntry */
use walkdir::DirEntry;

use utils::error;

/* Modules */
pub mod directory;
pub mod entry;

/* Constants */
/* -------------------------------------------- */
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
    "B",
    "KB",
    "MB",
    "GB",
    "TB",
    "PB",
    "EB",
    "ZB",
    "YB"
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
    "B",
    "KiB",
    "MiB",
    "GiB",
    "TiB",
    "PiB",
    "EiB",
    "ZiB",
    "YiB"
];
/* -------------------------------------------- */

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

pub fn get_nb_elements (p: &PathBuf) -> u64 {
    if p.is_dir() {
        match p.read_dir() {
            Ok(iter) => iter.count() as u64,
            Err(_) => 0u64,
        }
    }
    else { return 0u64 }
}

pub fn get_size(p: &PathBuf) -> u64 {
    match p.metadata() {
        Ok(metadata) => {
            metadata.len()
        }
        Err(_) => 0u64
    }
}

pub fn get_path_string(p: &PathBuf) -> String {
    match p.to_str() {
        Some(p_str) => p_str.to_string(),
        None => {
            error::err_msg("This PathBuf has no str !");
            error::exit(false)
        }
    }
}

pub fn get_systemtime(p: &PathBuf) -> SystemTime {
    match p.metadata() {
        Ok(metadata) => {
            match metadata.modified() {
                Ok(time) => time,
                Err(_) => SystemTime::now()
            }
        }
        Err(_) => SystemTime::now()
    }
}

pub fn get_datetime(p: &PathBuf) -> String {
    let datetime: DateTime<Local> = get_systemtime(p).into();
    datetime.format("%Y-%m-%d %T").to_string()
}

pub fn get_timestamp(p: &PathBuf) -> i64 {
    let datetime: DateTime<Local> = get_systemtime(p).into();
    datetime.timestamp()
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

pub fn is_hidden(path: &PathBuf) -> bool {
    get_filename(path).starts_with(".")
}
