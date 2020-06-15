use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
#[min_args(1)]
#[description("Let Mio say what you want")]
#[example("@user is great!")]
#[example("I like gween tea!")]
pub fn say(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| m.content(&args.rest()));
    let _ = msg.delete(&ctx.http);

    Ok(())
}

#[command]
#[min_args(1)]
#[description("Let Mio YELL IT OUT")]
#[example("@user is great!")]
#[example("I like gween tea!")]
pub fn yell(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.content(format!("**{}**", &args.rest().to_uppercase()))
    });
    let _ = msg.delete(&ctx.http);

    Ok(())
}
