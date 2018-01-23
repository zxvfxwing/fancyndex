use toml;
use toml::de::Error;
use io;

use std::path::Path;

/// "folder" section of Fancyndex.toml fileconf.
#[derive(Deserialize)]
pub struct Folder {
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
    pub folder: Folder,
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
                /// Inner test to watch if toml parsing is OK
                match toml::from_str(&data) {
                    Ok(cfg) => return cfg,
                    Err(e) => {
                        println!("Error while parsing TOML file {:} !\n{:?}", filename, e);
                        println!("Fancyndex will now use a default configuration.");
                        return Config::default()
                    }
                }
            },
            Err(e) => panic!("Error while reading {:} !\n{:?}", filename, e),
        }
    }

    /// Returns a default Config object.
    /// Triggered when TOML parsing fails.
    pub fn default() -> Config {
        return Config {
            folder: Folder {
                path: "..".to_string(),
            },
            walk_opt: WalkOpt {
                hidden: false,
                symlink: false,
            },
        }
    }

    /// Returns a Config object.
    ///
    /// # Arguments
    ///
    pub fn check(mut self) -> Config {
        if self.folder.path == "" {
            self.folder.path = "..".to_string();
        }

        if !Path::new(&self.folder.path).exists() {
            self.folder.path = "..".to_string();
        }

        return self
    }
}
