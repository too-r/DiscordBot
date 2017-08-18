use self::discord;
use self::discord::Discord;
use self::discord::model::{ServerId, Message};

pub fn ban(msg: Message, srv_id: ServerId, discord: Discord) {
    //The msg construct stores a list of mentions, the ban function takes the first mention and gets the id from there, meaning it is impossible to ban two users at once, you must run separate commands.
    let user_id = msg.mentions[0].id;
    
    match msg.author.id {
        &config.admins[0] => discord.add_ban(srv_id, user_id, 0), //Don't delete messages.
        &config.admins[1] => discord.add_ban(srv_id, user_id, 0),
        &config.admins[2] => discord.add_ban(srv_id, user_id, 0),
        &config.admins[3] => discord.add_ban(srv_id, user_id, 0),
        _ => {}, //We don't let anyone ban users except those users whose id's are specified in the config file.
    }
}