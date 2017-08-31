use toml;
use std::path::Path;
use std::env::home_dir;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
pub struct Config {
    pub token: Token,
    pub admins: Admins,
}

#[derive(Deserialize)]
pub struct Token {
    pub token: String,
}

#[derive(Deserialize)]
pub struct Admins {
    pub admins: Vec<u64>,
}

pub fn get_config() -> Config {
    let home_dir = home_dir().unwrap();

    let cfg_path = home_dir.join(".config").join("toorbot").join("config.toml");
    
    let mut file = File::open(cfg_path).unwrap();
    let mut buf = String::new();

    file.read_to_string(&mut buf).unwrap();

    toml::from_str(&buf).unwrap()
}
