use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use xdg;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub postgres: String,
    pub password: String,
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