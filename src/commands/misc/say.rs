use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
#[min_args(1)]
#[description("Let Aoyama say what you want")]
#[example("@user is great!")]
#[example("I like gween tea!")]
pub async fn say(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| m.content(&args.rest()))
        .await;
    let _ = msg.delete(&ctx.http);

    Ok(())
}

#[command]
#[min_args(1)]
#[description("Let Aoyama YELL IT ><")]
#[example("@user is great!")]
#[example("I like gween tea!")]
pub async fn yell(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content(format!("**{}**", &args.rest().to_uppercase()))
        })
        .await;
    let _ = msg.delete(&ctx.http);

    Ok(())
}
