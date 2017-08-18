#![feature(custom_derive, custom_attribute, stmt_expr_attributes)]
extern crate discord;
extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use discord::{Discord, State, Error, Connection};
use discord::model::{Event, ReadyEvent, ChannelId};
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::prelude::*;

mod commands;

pub fn main() {
    //Config file, should be under <Project dir>/config.json
    let mut file = File::open("config.json").unwrap();
    let mut config = String::new();
    file.read_to_string(&mut config).unwrap();

    #[derive(Deserialize)]
    pub struct Config {
        pub token: String,
    }

    //Parse JSON to the Config struct
    let config_json = serde_json::from_str::<Config>(&config).unwrap();
    let token = config_json.token;

    //Login to API
    let discord = Discord::from_bot_token(&token).expect("Expected a token");
    //Establish a websocket connection
    let (mut connection, ready) = discord.connect().expect("Could not connect");
    println!("[Ready] {} is serving {} servers",
             ready.user.username,
             ready.servers.len());
    //Object to track user state
    let mut state = State::new(ready);
    println!("{:?}", state.all_servers()[0]);


    //Receive events forever
    loop {
        let event = match connection.recv_event() {
            Ok(event) => event, //We received the event without errors
            Err(err) => {
                warning(&format!("[Warning] Receive error: {:?}", err));
                if let Error::WebSocket(..) = err {
                    //handle a connection drop
                    let (new_connection, ready) = discord.connect().expect("connect failed");
                    connection = new_connection;
                    state = State::new(ready);
                    println!("[Ready] Reconnected successfully.");
                }
                if let Error::Closed(..) = err {
                    break;   
                }
                continue;
            }
        };
        state.update(&event);

        match event {
            Event::MessageCreate(message) => {
                use std::ascii::AsciiExt;
                //Don't want to be replying to our own messages
                if message.author.id == state.user().id {
                    continue;
                }

                //Split the message into espaced segments
                let mut split = message.content.split(' ');
                let first_word = split.next().unwrap_or("");
                let argument = split.next().unwrap_or("");

                match first_word {
                    "!help" => {
                        if argument.eq_ignore_ascii_case("dj") {
                            discord.send_message(message.channel_id, "Plays YouTube videos in voice chat, ``!dj <link>``, ``!dj stop`` to stop playback ``!dj quit`` to stop and leave voice", "", false);
                        } else {
                            discord.send_message(message.channel_id, "Available commands: ``!dj``. Use ``!help <command>`` for more help on a command", "", false);
                        }
                    }
                    "!dj" => {
                        let vchan = state.find_voice_user(message.author.id);
                        if argument.eq_ignore_ascii_case("stop") {
                            vchan.map(|(sid, _)| connection.voice(sid).stop());
                        } else if argument.eq_ignore_ascii_case("quit") {
                            vchan.map(|(sid, _)| connection.drop_voice(sid));
                        } else {
                            let output = if let Some((server_id, channel_id)) = vchan {
                                match discord::voice::open_ytdl_stream(argument) {
                                    Ok(stream) => {
                                        let voice = connection.voice(server_id);
                                        voice.set_deaf(true);
                                        voice.connect(channel_id);
                                        voice.play(stream);
                                        String::new()
                                    }
                                    Err(error) => format!("Error: {}", error),
                                }
                            } else {
                                "You must be in a voice channel to DJ".to_owned()
                            };
                            if output.is_empty() {
                                warn(discord.send_message(message.channel_id, &output, "", false));
                            }
                        }
                    }
                    "!src" => {
                        discord.send_message(message.channel_id, "https://github.com/too-r/discordbot", "", false);
                    }
                    _ => continue
                }
            }
            Event::VoiceStateUpdate(server_id, _) => {
                if let Some(cur_channel) = connection.voice(server_id).current_channel() {
                    match server_id {
                        Some(server_id) => if let Some(srv) = state.servers().iter().find(|srv| srv.id == server_id) {
                            if srv.voice_states.iter().filter(|vs| vs.channel_id == Some(cur_channel)).count() <= 1 {
                                connection.voice(Some(server_id)).disconnect();
                            }
                        },
                        None => if let Some(call) = state.calls().get(&cur_channel) {
                            if call.voice_states.len() <= 1 {
                                connection.voice(server_id).disconnect();
                            }
                        }
                    }
                }
            }
            _ => {} //Discard other events
        }
    }
}

fn warn<T, E: ::std::fmt::Debug>(result: Result<T, E>) {
    match result {
        Ok(_) => {},
        Err(err) => println!("[Warning] {:?}", err),
    }
}

pub fn warning(output: &str) {
    println!("[Warning] {}", output);
}