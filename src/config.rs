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
    let home_path = home_dir().unwrap();
    let home_str = home_path.to_str().unwrap();

    let cfg_path = format!("{}/.config/toorbot/config.toml", home_str);

    let mut buf = String::new();
    let mut f = File::open(&cfg_path).unwrap();

    f.read_to_string(&mut buf).unwrap();
    toml::from_str(&buf).unwrap()
}
