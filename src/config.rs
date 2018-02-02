use toml;
use std::path::Path;

use io;
use filesystem::{pbuf_string, pbuf_parent_cdir};

#[derive(Deserialize)]
pub struct Root {
    pub path: String,
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
                path: pbuf_string(&pbuf_parent_cdir()),
            },
            walk_opt: WalkOpt {
                hidden: false,
                symlink: false,
            },
        }
    }

    /// Returns a correct Config object.
    ///
    /// # Arguments
    ///
    pub fn check(mut self) -> Config {
        let mut flag = false;

        /* Check all possible wrong cases */
        if self.root.path == "" { flag = true }
        if !flag {
            let p = Path::new(&self.root.path);
            if !p.exists() { flag = true }
            else {
                if !p.metadata().unwrap().is_dir() {
                    flag = true;
                }
            }
        }

        if flag {
            self.root.path = pbuf_string(&pbuf_parent_cdir());
        }

        return self
    }
}
