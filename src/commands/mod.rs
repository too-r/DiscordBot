use serenity::utils::Colour;

command!(about(_context, message) {
    let _ = message.channel_id.say("A simple bot for now, with basic voice capability");
});

command!(ping(_context, message) {
    let _ = message.channel_id.say("Pong!");
});

//User info.
command!(info(_context, message) {
    if let Some(guild) = message.guild() {
        let guild = guild.read().expect("Could not acquire a read lock");
        
        //If this user is still a member of the guild that the message was sent in
        if let Some(member) = guild.members.get(&message.author.id) {
            let mut roles = "@every\u{200b}one".to_owned();
            let mut iter = member.roles.iter();
            while let Some(role_id)= iter.next() {
                if let Some(role) = role_id.find() {
                    roles.push_str(", ");
                    roles.push_str(&role.name);
                } else {
                    //There was no corresponding RoleId for this Role. Return an error.
                    return Err("Failed to get Role for RoleId".to_owned());
                }
            }

            let joined_at = {
                if let Some(join_date) = member.joined_at.as_ref() {
                    join_date.naive_utc().format("%c")
                } else {
                    return Err("Failed to get the date this user joined at".to_owned());
                }
            };
            let avatar_url = message.author.face();
            let id = message.author.id.0.to_string();
            let nick = member.nick.as_ref().unwrap_or_else(|| &message.author.name);
            let dtag = message.author.tag();
            let created_at = message.author.created_at().format("%c").to_string();
            let footer_text = format!("Member since {}", joined_at);

            //Main embed.
            let result = message.channel_id.send_message(|cm| cm.embed(|ce|
                ce.author(|cea| cea.name(&dtag).icon_url(&avatar_url))
                .title("Info")
                .field(|cef| cef.inline(true).name("Id").value(&id))
                .field(|cef| cef.inline(true).name("Current Name").value(nick))
                .field(|cef| cef.inline(true).name("Created at").value(&created_at))
                .field(|cef| cef.inline(true).name("Roles").value(&roles))
                .footer(|cef| cef.text(&footer_text))
                .image(&avatar_url)
                .color(Colour::from_rgb(124, 42, 42))
            ));
            if result.is_err() {
                return Err("Failed to send message".to_owned())
            }
        }
    }
}); 
