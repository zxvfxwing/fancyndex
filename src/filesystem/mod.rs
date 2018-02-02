use std::env;
use std::path::{Path,PathBuf};
use walkdir::DirEntry;

pub mod entries;
pub mod unsafepath;

pub fn pbuf_is_dir(p: &PathBuf) -> bool {
    p.exists() && p.is_dir()
}

/// Returns the PathBuf of the current directory/folder.
pub fn pbuf_cdir() -> PathBuf {
    match env::current_dir() {
        Ok(cdir) => cdir,
        Err(e) => {
            println!("Error while accessing current directory !\n {}", e);
            Path::new(".").to_path_buf()
        },
    }
}

/// Returns the parent of the PathBuf given.
pub fn pbuf_parent(p: &PathBuf) -> PathBuf {
    if let Some(parent) = p.parent() { parent.to_path_buf() }
    else                             { p.to_path_buf()  }
}

/// Returns parent of the current directory.
pub fn pbuf_parent_cdir() -> PathBuf {
    pbuf_parent(&pbuf_cdir())
}

pub fn pbuf_str(p: &PathBuf) -> &str {
    p.to_str().unwrap_or(".")
}

pub fn dir_e_is_symlink(entry: &DirEntry) -> bool {
    entry.file_type().is_symlink()
}

pub fn dir_e_is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

pub fn dir_e_name(entry: &DirEntry) -> String {
    entry.file_name()
         .to_os_string()
         .into_string()
         .unwrap_or("".to_string())
}

pub fn dir_e_size(entry: &DirEntry) -> u64 {
    if entry.file_type().is_file(){
        match entry.metadata() {
            Ok(metadata) => metadata.len(),
            Err(_) => 0u64,
        }
    }
    else { 0u64 }
}

pub fn dir_e_pbuf(entry: &DirEntry) -> PathBuf {
    entry.path().to_path_buf()
}