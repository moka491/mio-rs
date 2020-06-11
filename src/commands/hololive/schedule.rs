use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
pub fn schedule(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.say(ctx, "だいじょうぶ！");

    Ok(())
}
