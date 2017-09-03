#![feature(custom_derive, custom_attribute, stmt_expr_attributes)]
extern crate discord;
extern crate toml;
#[macro_use]
extern crate serde_derive;

pub mod config;
mod commands;

use discord::{Discord, State, Error, Connection, ChannelRef};
use discord::model::{Event, ReadyEvent, ChannelId};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use config::get_config;

pub fn main() {
    let token = get_config().token.token;

    //Login to API
    let discord = Discord::from_bot_token(&token).expect("Expected a token");
    //Establish a websocket connection
    let (mut connection, ready) = discord.connect().expect("Could not connect");
    println!("[Ready] {} is serving {} servers",
             ready.user.username,
             ready.servers.len());
    //Object to track user state
    let mut state = State::new(ready);

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

                //Just a bit of a safeguard to ensure we don't reply to our own messages.
                if message.author.id == state.user().id {
                    continue
                }

                //Create a config object to pass to all our functions.
                let mut config = get_config();

                let mut split = message.content.split(' ');
                let first_word = split.next().unwrap_or("");
                let argument = split.next().unwrap_or("");

                match first_word {
                    "~help" => commands::help(&discord, &message, argument),
                    "~ban" => {
                        match state.find_channel(message.channel_id).unwrap() {
                            ChannelRef::Public(ref server, _) => {
                                commands::admin::ban(&message, server.id, &discord, &config);
                            },
                            _ => {},
                        }
                    }

                    "kick" => {
                        match state.find_channel(message.channel_id).unwrap() {
                            ChannelRef::Public(ref server, _) => {
                                commands::admin::kick(&message, server.id, &discord, &config);
                            },
                            
                            _ => {},
                        }
                    }
                    _ => continue,
                }
            }

            Event::VoiceStateUpdate(server_id, _) => {
                if let Some(cur_channel) = connection.voice(server_id).current_channel() {
                    match server_id {
                        Some(server_id) => {
                            if let Some(srv) = state.servers().iter().find(|srv| srv.id == server_id) {
                                if srv.voice_states
                                       .iter()
                                       .filter(|vs| vs.channel_id == Some(cur_channel))
                                       .count() <= 1 {
                                    connection.voice(Some(server_id)).disconnect();
                                }
                            }
                        }
                        None => {
                            if let Some(call) = state.calls().get(&cur_channel) {
                                if call.voice_states.len() <= 1 {
                                    connection.voice(server_id).disconnect();
                                }
                            }
                        }
                    }
                }
            }
            _ => {} //Discard other events
        }
    }
}

pub fn warn<T, E: ::std::fmt::Debug>(result: Result<T, E>) {
    match result {
        Ok(_) => {}
        Err(err) => println!("[Warning] {:?}", err),
    }
}

pub fn warning(output: &str) {
    println!("[Warning] {}", output);
}
