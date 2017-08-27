use discord::{Discord, State, model::Message};

pub fn dj(discord: &Discord, msg: &Message, state: &State, arg: String) {
    let vchan = state.find_voice_user(&msg.author.id);
    
}
