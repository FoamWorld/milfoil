use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct EnvConfig {
    pub font: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaConfig {
    pub preload: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub env: EnvConfig,
    pub lua: LuaConfig,
}

pub fn load_config() -> Config {
    let config_contents = match fs::read_to_string("./assets/config/app.toml") {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading config file: {}", err);
            panic!();
        }
    };

    match toml::from_str(&config_contents) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error parsing config file: {}", err);
            panic!();
        }
    }
}
