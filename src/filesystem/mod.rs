use std::env;
use std::path::{Path,PathBuf};
use walkdir::DirEntry;
use std::ffi::OsString;

pub mod entries;

/// Returns the PathBuf of the current directory/folder.
pub fn cdir() -> PathBuf {
    match env::current_dir() {
        Ok(cdir) => cdir,
        Err(e) => {
            println!("Error while accessing current directory !\n {}", e);
            Path::new(".").to_path_buf()
        },
    }
}

/// Returns the parent of the PathBuf given.
pub fn parent_dir(p: &PathBuf) -> PathBuf {
    if let Some(parent) = p.parent() { parent.to_path_buf() }
    else                             { p.to_path_buf()  }
}

/// Returns parent of the current directory.
pub fn parent_cdir() -> PathBuf {
    parent_dir(&cdir())
}

/// Returns the path string of a PathBuf.
pub fn path_string(p: &PathBuf) -> String {
    if let Some(p_str) = p.to_str() { p_str.to_string() }
    else                            { ".".to_string() }
}

pub fn is_symlink(entry: &DirEntry) -> bool {
    entry.file_type().is_symlink()
}

pub fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

pub fn get_file_name(entry: &DirEntry) -> Result<String, OsString> {
    entry.file_name().to_os_string().into_string()
}

pub fn get_file_size(entry: &DirEntry) -> u64 {
    if let Ok(metadata) = entry.metadata() {
        metadata.len()
    }
    else {
        0u64
    }
}