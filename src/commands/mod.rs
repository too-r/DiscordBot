pub mod admin;
pub mod music;

use config::get_config;
use discord::Discord;
use discord::model::Message;

struct PossibleHelpCases {
    music: String,
    admin: String,
    ban: String,
    kick: String,
}

pub fn help(discord: Discord, msg: Message, arg: &'static str) {
    //Grab us a config object
    let config = get_config();

    match arg {
        "music" => {
            discord.send_message(msg.channel_id, "Play music using ``~dj <YouTube link>``. ``~dj stop`` stops playback, and ``~dj quit`` makes the bot leave the channel.", "", false);
        }

        "admin" => {
            discord.send_message(msg.channel_id, 
                                 "Authorized users can use priveleged commands. The available commands are ``~ban`` and ``~kick`` Use ``~help ban`` or ``~help kick`` to get more info on the synax of these", "", false);
        }

        "ban" => {
            discord.send_message(msg.channel_id, "Usage: ``~ban <mention>``", "", false);
        }

        "kick" => {
            discord.send_message(msg.channel_id, "Usage: ``~ban <mention>``", "", false);
        }

        //For any other argument, just send the help command
        _ => {
            discord.send_message(msg.channel_id, "Get help on specific commands using ``~help <command>``. Modules are ``admin``, ``music``", "", false);
        }
    }
}
