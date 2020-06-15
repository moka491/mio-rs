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
    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| m.content(&args.rest()));

    Ok(())
}
