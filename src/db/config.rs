use std::io::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};

use xdg;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub port: u16,
    pub postgres: String,
    pub passhash: String,
    pub root: PathBuf,
}

impl Config {
    pub fn load() -> Result<Config, ()> {
        let basedirs = xdg::BaseDirectories::new().unwrap();
        let mut file = String::new();
        File::open(basedirs.get_config_home().join(Path::new("storier/storier.cfg"))).unwrap().read_to_string(&mut file).unwrap();

        if let Ok(config) = toml::from_str(&file) {
            Ok(config)
        } else {
            Err(())
        }
    }
}