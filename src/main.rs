#![feature(custom_derive, custom_attribute, stmt_expr_attributes)]
extern crate discord;
extern crate toml;
#[macro_use]
extern crate serde_derive;

pub mod config;
mod commands;
mod handlers;

use discord::{Discord, State, Error, Connection, ChannelRef};
use discord::model::{Event, ReadyEvent, ChannelId};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use config::get_config;

static mut RATE_LIMITED: bool = false;
static mut RETRY_TIME: u64 = 1;

fn main() {
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
                warning(&format!("Receive error: {:?}", err));
                if let Error::WebSocket(..) = err {
                    //handle a connection drop
                    let (new_connection, ready) = discord.connect().expect("connect failed");
                    connection = new_connection;
                    state = State::new(ready);
                    println!("[Ready] Reconnected successfully.");
                }
                
                //A ratelimit error, inform the user that they were ratelimited and how long they
                //have to retry.
                if let Error::RateLimited(value) = err {
                    unsafe {
                        RATE_LIMITED = true;
                    }

                    unsafe {
                        RETRY_TIME = value;
                    }
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
                //Tell the user to stop if we are being ratelimited.
                unsafe { 
                    if RATE_LIMITED {
                        let mut secs = std::time::Duration::from_millis(RETRY_TIME).as_secs();
                        let warning_str = &format!("I am being ratelimited. Please retry in {} seconds", secs);
                        discord.send_message(message.channel_id, warning_str, "", false);
                    } else {
                        if message.author.id == state.user().id {
                            //The message was from us, restart the loop.
                            continue
                        }
                    
                        //Handle this message.
                        handlers::message_handler(&state, &discord, &message, connection);
                    }
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
