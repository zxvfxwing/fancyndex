use std::io::Error;
use std::io::prelude::*;
use std::fs::File;

pub fn read_file(filename: &str) -> Result<String, Error> {
    let mut file = File::open(filename)?;
    let mut file_str = String::new();
    file.read_to_string(&mut file_str)?;
    Ok(file_str)
}
