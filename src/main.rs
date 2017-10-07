#[macro_use]
extern crate serenity;
extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate chrono;

mod commands;

use serenity::prelude::*;
use serenity::model::*;
use serenity::framework::standard::{Args, Command, DispatchError, StandardFramework, help_commands};
use std::env;

struct Handler;

impl EventHandler for Handler {
    fn on_ready(&self, _: Context, ready: Ready) {
        println!("{} is connected. Serving {} servers", ready.user.name, ready.guilds.len());
    }
}

fn main() {
    let mut token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    
    let mut client = Client::new(&token, Handler);

    client.with_framework(
        StandardFramework::new()
        .configure(|c| c
            .on_mention(true)
            .prefix("~"))
        .group("Help", |g| g
            .command("help", |c| c
                .exec_help(help_commands::with_embeds)
                .desc("Displays a list of the bot commands with embeds")))
        .group("Meta", |g| g
            .command("info", |c| c
                .exec(commands::info)
                .desc("Displays user info")))
    );

    if let Err(err) = client.start() {
        println!("Client error: {}", err);
    }
}
