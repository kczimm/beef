use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{Read, Write as _},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config(HashMap<String, String>);

impl Config {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn load(&mut self) {
        if let Some(path) = get_path() {
            // try to load the config
            let mut config = String::new();
            if let Ok(mut f) = File::open(&path) {
                f.read_to_string(&mut config).unwrap();
                *self = toml::from_str(&config).unwrap();
            }
        }
    }

    pub fn save(&mut self) {
        if let Some(path) = get_path() {
            if let Ok(mut f) = File::open(&path) {
                let config = toml::to_string_pretty(&self).unwrap();
                f.write_all(config.as_bytes()).unwrap();
                f.flush().unwrap();
            }
        }
    }
}

fn get_path() -> Option<String> {
    Some(match env::var("BEEF_CONFIG_PATH") {
        Ok(path) => path,
        Err(_) => match env::var("HOME") {
            Ok(home) => format!("{home}/.config/beef/config.toml"),
            Err(_) => return None,
        },
    })
}
