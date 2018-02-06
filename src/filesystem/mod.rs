use std::env;
use std::path::{Path,PathBuf};
use std::ffi::OsStr;

pub mod entries;
pub mod unsafepath;
pub mod walkdir;

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

pub fn pbuf_is_hidden(p: &PathBuf) -> bool {
    p.file_name()
     .unwrap_or(OsStr::new(""))
     .to_str()
     .map(|s| s.starts_with("."))
     .unwrap_or(false)
}

pub fn pbuf_is_symlink(p: &PathBuf) -> bool {
    match p.symlink_metadata() {
        Ok(md) => md.file_type().is_symlink(),
        Err(_) => false,
    }
}

pub fn pbuf_vstring(p: &PathBuf) -> Vec<String> {
     let vstr: Vec<&str> = p.to_str()
                            .unwrap()
                            .split("/")
                            .collect();

    let vstring: Vec<String> = vstr.iter()
                                   .map(|s| s.to_string())
                                   .collect();

    vstring
} 