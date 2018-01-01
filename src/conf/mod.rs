use toml;
use utils::error;
use utils::io;

#[derive(Deserialize)]
pub struct WalkOpt {
    pub do_hidden: bool,
    pub do_symlink: bool,
}

#[derive(Deserialize)]
pub struct UnitOpt {
    pub binary_unit: bool
}

#[derive(Deserialize)]
pub struct Config {
    pub walk_options: WalkOpt,
    pub unit_options: UnitOpt,
}

pub fn read_cfg_file(filename: &str) -> Config {
    match io::read_file(filename) {
        Ok(s) => {
            match toml::from_str(&s[..]) {
                Ok(cfg) => cfg,
                Err(e) => {
                    error::err_msg("Error into configuration file !");
                    error::err_msg(&e.to_string()[..]);
                    error::exit(false);
                }
            }
        },
        Err(e) => {
            error::err_msg("Error occurred while trying to read configuration file.");
            error::err_msg(&e.to_string()[..]);
            error::exit(false);
        },
    }
}
