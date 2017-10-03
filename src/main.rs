#[macro_use] 
extern crate serenity;
extern crate toml;

use serenity::prelude::*;
use serenity::model::*;
use serenity::framework::StandardFramework;
use std::env;
use std::path::Path;
use std::fs::File;

mod commands;

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

command!(about(_context, message) {
    let _ = message.channel_id.say("A simple bot for now, with basic voice capability");
});

command!(ping(_context, message) {
    let _ = message.channel_id.say("Pong!");
});
