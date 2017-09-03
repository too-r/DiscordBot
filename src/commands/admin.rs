use discord::model::{Message, ServerId};
use discord::Discord;
use std::fs::File;
use std::path::Path;
use config::Config;

pub fn ban(msg: &Message, srv_id: ServerId, discord: &Discord, config: &Config) {
    let user_id = msg.mentions[0].id;

    //Make this a u64 just so we can check for it in the admins Vec.
    let user_id_u64 = format!("{}", user_id).parse::<u64>().unwrap();

    let author_id = format!("{}", msg.author.id).parse::<u64>().unwrap(); //Parse id to string and back again

    if config.admins.admins.contains(&user_id_u64) {
        discord.send_message(msg.channel_id,
                             "Cannot ban a user who is set as an admin",
                             "",
                             false);
    } else {
        if config.admins.admins.contains(&author_id) {
            discord.add_ban(srv_id, user_id, 10).unwrap();
            discord.send_message(msg.channel_id, "Banned the mentioned user", "", false);
        } else {
            let message = format!("{} You do not have permission to do that",
                                  user_id.mention());
            discord.send_message(msg.channel_id, &message, "", false);
        }
    }
}

pub fn kick(msg: &Message, srv_id: ServerId, discord: &Discord, config: &Config) {
    let user_id = msg.mentions[0].id;

    let user_id_u64 = format!("{}", user_id).parse::<u64>().unwrap();

    let author_id = format!("{}", msg.author.id).parse::<u64>().unwrap();

    if config.admins.admins.contains(&user_id_u64) {
        discord.send_message(msg.channel_id,
                             "Cannot kick a user who is set as an admin",
                             "",
                             false);
    } else {
        if config.admins.admins.contains(&author_id) {
            discord.kick_member(srv_id, user_id);
        } else {
            let message = format!("{} You do not have permission to do that",
                                  user_id.mention());
            discord.send_message(msg.channel_id, &message, "", false);
        }
    }
}
