use toml;
use toml::de::Error;
use io;

use std::path::Path;

/// "folder" section of Fancyndex.toml fileconf.
#[derive(Deserialize)]
pub struct Folder {
    pub path: String,
}

/// Config abstract object. Respresents fileconf itself.
#[derive(Deserialize)]
pub struct Config {
    pub folder: Folder,
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
                    Err(e) => panic!("Error while parsing TOML file {:} !\n{:?}", filename, e),
                }
            },
            Err(e) => panic!("Error while reading {:} !\n{:?}", filename, e),
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

        return self;
    }
}
