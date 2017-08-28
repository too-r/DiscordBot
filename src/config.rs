use toml;
use std::path::Path;
use std::env::home_dir;

#[derive(Deserialize)]
pub struct Config {
    pub token: String,
    pub admins: Vec<u64>,
}

pub fn get_config(p: Path) -> Config {
    let home_path = home_dir.unwrap();
    let home_str = home_path.to_str().unwrap();

    let cfg_path = format!("{}./{}", home_str, p);
    
    let mut buf = String::new();
    let mut f = File::open(&cfg_path).unwrap();
    
    f.read_to_string(&mut buf).unwrap();
    toml::from_str(&buf).unwrap()
}
