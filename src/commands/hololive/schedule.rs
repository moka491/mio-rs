use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
pub fn schedule(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.reply(ctx, "Hello");

    Ok(())
}
