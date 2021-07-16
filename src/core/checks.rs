use serenity::{
    client::Context,
    framework::standard::{macros::check, Args, CommandOptions, Reason},
    model::channel::Message,
};

#[check]
#[name = "IsNSFW"]
async fn nsfw_check(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    match msg.channel_id.to_channel(&ctx).await.unwrap().is_nsfw() {
        true => Ok(()),
        false => Err(Reason::User(
            "This command can only be used in nsfw-enabled channels!".to_string(),
        )),
    }
}

#[check]
#[name = "IsAdmin"]
async fn is_admin(
    ctx: &mut Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    if let Some(member) = msg.member(&mut ctx.cache).await {
        if let Ok(permissions) = member.permissions(&ctx.cache) {
            return permissions.administrator().into();
        }
    }

    false.into()
}
