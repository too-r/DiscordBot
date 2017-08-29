use discord::model::{Message, ServerId};
use discord::Discord;
use std::fs::File;
use std::path::Path;
use super::config::Config;

pub fn ban(msg: Message, srv_id: ServerId, discord: Discord, config: &Config) {
    let config = config::get_config(); //Config object

    //Take the id of the user we want to ban from the first mentioned user in the message.
    let user_id = msg.mentions[0].id;

    let author_id = format!("{}", msg.author.id).parse::<u64>().unwrap(); //Parse id to string and back again

    if config.admins.contains(user_id) {
        discord.send_message(msg.channel_id, "Cannot ban a user who is set as an admin", "", false);
    } else {
        if config.admins.contains(author_id) {
            discord.add_ban(srv_id, user_id);
        } else {
            let message = format!("{} You do not have permission to do that", user_id.mention());
            discord.send_message(msg.channel_id, message, "", false);
        }
    }
}

pub fn kick(msg: Message, srv_id: ServerId, discord: Discord, config: &Config) {
    let user_id = msg.mentions[0].id;

    let author_id = format!("{}", msg.author.id).parse::<u64>().unwrap();
    
    if config.admins.contains(user_id) {
        discord.send_message(msg.channel_id, "Cannot kick a user who is set as an admin", "", false);
    } else {
        if config.admins.contains(author_id) {
            discord.add_ban(srv_id, user_id);
        } else {
            let message = format!("{} You do not have permission to do that", user_id.mention());
            discord.send_message(msg.channel_id, message, "", false);
        }
    }    
}
