use serenity::{
    client::Context,
    framework::standard::{macros::check, Args, CheckResult, CommandOptions, Reason},
    model::channel::Message,
};

#[check]
#[name = "IsNSFW"]
async fn nsfw_check(ctx: &Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    match msg.channel_id.to_channel(&ctx).await.unwrap().is_nsfw() {
        true => CheckResult::Success,
        false => CheckResult::Failure(Reason::User(
            "This command can only be used in nsfw-enabled channels!".to_string(),
        )),
    }
}
