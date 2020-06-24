use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    http::AttachmentType,
    model::channel::Message,
    prelude::Context,
};
use std::borrow::Cow;

#[command]
#[description("Generate a list of all the images posted recently")]
pub fn collect(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let mut link_list: Vec<String> = vec![];

    let _messages: Vec<Message> = msg
        .channel_id
        .messages(&ctx.http, |retriever| retriever.before(msg.id))?;

    for message in _messages {
        message
            .attachments
            .iter()
            .filter(|a| a.width.is_some())
            .for_each(|a| link_list.push(a.url.clone()));

        message
            .embeds
            .iter()
            .filter_map(|e| e.image.clone())
            .for_each(|i| link_list.push(i.url.clone()));
    }

    if !link_list.is_empty() {
        let file_content = link_list.join("\n");
        let file_bytes = file_content.as_bytes();

        let attachment: AttachmentType = AttachmentType::Bytes {
            data: Cow::from(file_bytes),
            filename: "filename.txt".to_string(),
        };

        let _ = msg
            .channel_id
            .send_message(&ctx.http, |m| m.add_file(attachment));
    }

    Ok(())
}
