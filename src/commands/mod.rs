pub mod admin;
pub mod music;

use config::get_config;
use discord::{Discord, State, ChannelRef, Error};
use discord::model::Message;

pub fn help(discord: &Discord, msg: &Message, arg: &str) {
    //Grab us a config object
    let config = get_config();

    match arg {
        "music" => {
            discord.send_message(msg.channel_id, "Play music using ``~dj <YouTube link>``. ``~dj stop`` stops playback, and ``~dj quit`` makes the bot leave the channel.", "", false);
        }

        "admin" => {
            discord.send_message(msg.channel_id, 
                                 "Authorized users can use priveleged commands. The available commands are ``~ban`` and ``~kick``. Use ``~help ban`` or ``~help kick`` to get more info on the syntax of these.", "", false);
        }

        "ban" => {
            discord.send_message(msg.channel_id, "Usage: ``~ban <mention>``.", "", false);
        }

        "kick" => {
            discord.send_message(msg.channel_id, "Usage: ``~ban <mention>``.", "", false);
        }

        //For any other argument, just send the help command
        _ => {
            discord.send_message(msg.channel_id, "Get help on specific commands using ``~help <command>``. Modules are ``admin`` and ``music``.", "", false);
        }
    }
}

/*pub struct Info;

impl Info {
    pub fn server_info(discord: &Discord, state: &State, msg: &Message) -> Result<Message, Error> {
        match state.find_channel(msg.channel_id).unwrap() {
            ChannelRef::Public(ref server, _) => {
                let owner_id = server.owner_id;
                let region = server.region;

                if let Some(icon) = server.icon {
                    icon
                } else {
                    println!("No server icon found");
                }

                let message = discord.send_embed(/*Some shit here*/);
                message
            },

            _ => println!("The message was sent in a group or DM."),
        }
    }
}*/
