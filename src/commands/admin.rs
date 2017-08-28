use discord::model::{Message, ServerId};
use discord::Discord;
use std::fs::File;
use std::path::Path;
use super::config::Config;

pub fn ban(msg: Message, srv_id: ServerId, discord: Discord, config: &Config) {
    //The msg construct stores a list of mentions, the ban function takes the first mention and gets the id from there, meaning it is impossible to ban two users at once, you must run separate commands.
    let user_id = msg.mentions[0].id;
}

pub fn kick(msg: Message, srv_id: ServerId, discord: Discord, config: &Config) {
    let user_id = msg.mentions[0].id;

    let author_id = format!("{}", msg.author.id).parse::<u64>().unwrap();
}
