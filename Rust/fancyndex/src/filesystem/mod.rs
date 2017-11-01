pub mod directory;
pub mod file;

use chrono::prelude::*;
use std::path::PathBuf;

/* Debug atm */
fn get_current_timestamp() -> i64 {
    Local::now().timestamp()
}

fn get_current_datetime(datetime_format: &str) -> String {
    println!("datetime not found, so we take default one :");
    Local::now().format(datetime_format).to_string()
}

fn get_filename(p: &PathBuf) -> String {
    match p.file_name() {
        None => panic!("No file_name found !"),
        Some(filename) => match filename.to_str() {
            None => panic!("No str in file_name !"),
            Some(filename_str) => filename_str.to_string(),
        }
    }
}

fn get_size(p: &PathBuf) -> u64 {
    match p.metadata() {
        Ok(metadata) => metadata.len(),
        Err(_) => 0u64,
    }
}

fn get_timestamp(p: &PathBuf) -> i64 {
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

fn get_datetime(p: &PathBuf) -> String {
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
