use lazy_static::lazy_static;
use serenity::{
    framework::standard::{macros::command, Args, CommandError, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
#[min_args(1)]
#[description("Let Mio say what you want")]
#[example("@user is great!")]
#[example("I like gween tea!")]
pub fn say(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.content(&args.rest());
        m
    });

    Ok(())
}

#[command]
#[min_args(1)]
#[description("Let Mio say what you want")]
#[example("2")]
#[example("Whatever someone said")]
#[example("@user")]
#[example("@user 2")] // number is the amount of messages to jump back
#[example("@user Whatever the user said")] // search for the text a user said
pub fn quote(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    Ok(())
}
