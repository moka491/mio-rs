use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
#[description("Show bot and system information")]
pub fn info(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id
        .send_message(&ctx.http, |m| m.embed(|e| e.title("test").description("こんばんみぉーん！ I'm Ookami Mio, your fellow helper bot written in Rust using [serenity](https://github.com/serenity-rs/serenity)")));

    Ok(())
}
