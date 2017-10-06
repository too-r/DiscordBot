#[macro_use] 
extern crate serenity;
extern crate toml;
extern crate chrono;

mod commands;

use serenity::prelude::*;
use serenity::model::*;
use serenity::framework::StandardFramework;
use std::env;
use std::path::Path;
use std::fs::File;
use commands::{about, ping};

struct Handler;

impl EventHandler for Handler {
    fn on_ready(&self, _: Context, ready: Ready) {
        println!("Connected with name: {}", ready.user.name);
        println!("Serving {} servers", ready.guilds.len());
    }
}

fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler);

    client.with_framework(
        StandardFramework::new()
        .configure(|c| c.prefix("~"))               
        .command("about", |c| c.exec(about))
        .command("ping", |c| c.exec(ping)));

    if let Err(client_err) = client.start() {
        println!("Client error: {:?}", client_err);
    }
}

/*#[allow(dead_code)]
fn setup() -> Result<String, ()> {
    let mut path: Path = "$HOME/.config/toorbot/config.toml";
    let mut f = File::open(&path)?;
    let mut buf = String::new();

    f.read_to_string(&mut buf)?;

    Ok(buf)
}*/
