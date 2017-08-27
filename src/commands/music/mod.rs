use discord::{Discord, State, model::Message, Connection};
use std::ascii::AsciiExt;

pub fn play(discord: Discord, msg: Message, state: State, arg: String, connection: Connection) {
    let vchan = state.find_voice_user(msg.author.id);
    if argument.eq_ignore_ascii_case("stop") {
        vchan.map(|(sid, _)| connection.voice(sid).stop());
    } else if argument.eq_ignore_ascii_case("quit") {
        vchan.map((|sid, _|) connection.drop_voice(sid));
    } else {
        let output = if let Some((server_id, channel_id)) = vchan {
            match discord::voice::open_ytdl_stream(argument) {
                Ok(stream) => {
                    let voice = connection.voice(server_id);
                    voice.set_deaf(true);
                    voice.connect(channel_id);
                    voice.play(stream);
                    String::new()
                }
                Err(error) => format!("Error: {}", error),
            } else {
                "You must be in a voice channel to play music".to_owned()
            };
            if output.is_empty() {
                warn(discord.send_message(message.channel_id, &output, "", false));
            }
        } 
    }
}
