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

#[derive(Deserialize)]
pub struct EntriesOpt {
    pub datetime_format: String,
    pub unit_size: bool,
    pub float_precision: usize,
}

#[derive(Deserialize)]
pub struct Config {
    pub root: Root,
    pub walk_opt: WalkOpt,
    pub entries_opt: EntriesOpt,
}

impl Config {
    /// Returns a Config object.
    ///
    /// # Arguments
    ///
    /// * `filename` - A String slice that holds the filename of the configuration file.
    pub fn new(filename: &str) -> Self {

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
    pub fn default() -> Self {
        return Config {
            root: Root {
                path: pbuf_parent_cdir(),
            },
            walk_opt: WalkOpt {
                hidden: false,
                symlink: false,
            },
            entries_opt: EntriesOpt {
                datetime_format: "%Y-%m-%d %T".to_string(),
                unit_size: true,
                float_precision: 2usize,
            }
        }
    }

    /// Correct the current config if it's necessary.
    pub fn check(mut self) -> Self {
        if !pbuf_is_dir(&self.root.path) {
            println!("Warning: the root.path into Fancyndex.toml doesn't exists or isn't a directory !");
            println!("root.path equals now to the Fancyndex parent folder :");
            self.root.path = pbuf_parent_cdir();
            println!("root.path = {} ", pbuf_str(&self.root.path)); 
        }
        self
    }
}
