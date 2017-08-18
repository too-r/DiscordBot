use self::discord;
use self::discord::Discord;
use self::discord::model::{ServerId, Message};

pub fn ban(msg: Message, srv_id: ServerId, discord: Discord) {
    let user_id = msg.mentions[0].id;
    
    match msg.author.id {
        &config.admins[0] => discord.add_ban(srv_id, user_id, 0);
        &config.admins[1] => discord.add_ban(srv_id, user_id, 0);
        &config.admins[2] => discord.add_ban(srv_id, user_id, 0);
        &config.admins[3] => discord.add_ban(srv_id, user_id, 0);
    }
}