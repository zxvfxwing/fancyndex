use toml;
use std::path::PathBuf;

use io;
use filesystem::{pbuf_is_dir, pbuf_str, pbuf_parent_cdir};

#[derive(Deserialize)]
pub struct Root {
    pub path: PathBuf,
}

#[derive(Deserialize)]
pub struct WalkOpt {
    pub hidden: bool,
    pub symlink: bool,
}

/// Config abstract object. Respresents fileconf itself.
#[derive(Deserialize)]
pub struct Config {
    pub root: Root,
    pub walk_opt: WalkOpt,
}

impl Config {
    /// Returns a Config object.
    ///
    /// # Arguments
    ///
    /// * `filename` - A String slice that holds the filename of the configuration file.
    pub fn new(filename: &str) -> Config {

        match io::read_file(filename) {
            Ok(data) => {
                match toml::from_str(&data) {
                    Ok(cfg) => cfg,
                    Err(e) => {
                        println!("Error while parsing TOML file {} !\n{}", filename, e);
                        println!("Fancyndex will now use a default configuration.");
                        Config::default()
                    }
                }
            },
            Err(e) => {
                println!("Error while reading {:} !\n{}", filename, e);
                println!("Fancyndex will now use a default configuration.");
                Config::default()
            }
        }
    }

    /// Returns a default Config object.
    /// Triggered when TOML parsing fails.
    pub fn default() -> Config {
        return Config {
            root: Root {
                path: pbuf_parent_cdir(),
            },
            walk_opt: WalkOpt {
                hidden: false,
                symlink: false,
            },
        }
    }

    /// Correct the current config if it's necessary.
    pub fn check(mut self) -> Config {
        if !pbuf_is_dir(&self.root.path) {
            println!("Warning: the root.path into Fancyndex.toml doesn't exists or isn't a directory !");
            println!("root.path equals now to the Fancyndex parent folder :");
            self.root.path = pbuf_parent_cdir();
            println!("root.path = {} ", pbuf_str(&self.root.path)); 
        }
        self
    }
}
