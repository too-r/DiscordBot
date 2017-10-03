command!(about(_context, message) {
    let _ = message.channel_id.say("A simple bot for now, with basic voice capability");
});

command!(ping(_context, message) {
    let _ = message.channel_id.say("Pong!");
});
