/*
* In/Out Module
*
* What is related to Read / Write file
*
*/

use std::io::prelude::*;
use std::io::Error;
use std::fs::File;
use std::io::BufReader;

pub fn read_file(filename: &str) -> Result<String, Error> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
