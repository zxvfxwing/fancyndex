pub mod directory;
pub mod file;

use chrono::prelude::*;
use std::path::PathBuf;
use std::env;
use std::process;

pub fn get_parent_current_dir() -> PathBuf {
    let current_path = get_current_directory();
    match current_path.parent() {
        Some(parent) => parent.to_path_buf(),
        None => {
            println!("We cannot read the current directory of fancyndex.");
            println!("Please check permissions !");
            println!("Exiting program ...");
            process::exit(1)
        }
    }
}

pub fn get_current_directory() -> PathBuf {
    match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            println!("{}", e.to_string());
            println!("We cannot read the current's parent directory of fancyndex.");
            println!("Please check permissions !");
            println!("Exiting program ...");
            process::exit(1)
        }
    }
}

/* Debug atm */
pub fn get_current_timestamp() -> i64 {
    Local::now().timestamp()
}

pub fn get_current_datetime(datetime_format: &str) -> String {
    Local::now().format(datetime_format).to_string()
}

pub fn get_filename(p: &PathBuf) -> String {
    match p.file_name() {
        None => panic!("No file_name found !"),
        Some(filename) => match filename.to_str() {
            None => panic!("No str in file_name !"),
            Some(filename_str) => filename_str.to_string(),
        }
    }
}

pub fn get_size(p: &PathBuf) -> u64 {
    match p.metadata() {
        Ok(metadata) => metadata.len(),
        Err(_) => 0u64,
    }
}

pub fn get_timestamp(p: &PathBuf) -> i64 {
    match p.metadata() {
        Ok(metadata) => {
            /*
            * Convert
            * std::time::SystemTime => chrono:: DateTime<Local>
            */
            match metadata.modified() {
                Ok(time) => {
                    let datetime: DateTime<Local> = time.into();
                    datetime.timestamp()
                },
                Err(_) => get_current_timestamp()
            }
        },
        Err(_) => get_current_timestamp()
    }
}

pub fn get_datetime(p: &PathBuf) -> String {
    match p.metadata() {
        Ok(metadata) => {
            /*
            * Convert
            * std::time::SystemTime => chrono:: DateTime<Local>
            */
            match metadata.modified() {
                Ok(time) => {
                    let datetime: DateTime<Local> = time.into();
                    datetime.format("%Y-%m-%d %T").to_string()
                },
                Err(_) => get_current_datetime("%Y-%m-%d %T")
            }
        },
        Err(_) => get_current_datetime("%Y-%m-%d %T")
    }
}
