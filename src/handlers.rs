extern crate std;
use discord::{Discord, State, ChannelRef, Connection};
use discord::model::Message;
use {RATE_LIMITED, RETRY_TIME};
use config::get_config;
use commands;

pub fn message_handler(state: &State, discord: &Discord, message: &Message, connection: Connection) {
    //Tell the user to stop if we are being ratelimited.
    if RATE_LIMITED {
        let mut secs = std::time::Duration::from_millis(RETRY_TIME).as_secs();
        let warning_str = &format!("I am being ratelimited. Please retry in {} seconds", secs);
        discord.send_message(message.channel_id, warning_str, "", false);
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
        "~kick" => {
            match state.find_channel(message.channel_id).unwrap() {
                ChannelRef::Public(ref server, _) => {
                    commands::admin::kick(&message, server.id, &discord, &config);
                },
                            
                _ => {},
            }
        }
        "~dj" => commands::music::play(&discord, &message, &state, argument, &mut connection),

        _ => return,
    }
}
