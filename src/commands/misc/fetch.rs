use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    http::AttachmentType,
    model::{channel::Message, id::MessageId},
    prelude::Context,
};
use std::borrow::Cow;

const MAX_MESSAGE_COUNT: u32 = 5000;
const REQUESTS_PER_ITER: u64 = 100;

#[command]
#[description("Generate a list of all the images posted recently")]
pub fn fetch(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut link_list: Vec<String> = vec![];

    let mut last_message_id: MessageId = msg.id;
    let mut message_counter: u32 = 0;
    let mut end_reached = false;

    let id_arg = args.single::<u64>().unwrap_or_default();

    while !end_reached {
        let _messages: Vec<Message> = msg.channel_id.messages(&ctx.http, |retriever| {
            retriever.before(last_message_id).limit(REQUESTS_PER_ITER)
        })?;

        message_counter += _messages.len() as u32;
        last_message_id = _messages.last().unwrap().id;

        // If the retrieved messages are less than what expected (usually means we reached the beginning of the history)
        // or we reached a max amount of requests to make, stop after this iteration
        if _messages.len() < REQUESTS_PER_ITER as usize || message_counter > MAX_MESSAGE_COUNT {
            end_reached = true;
        }

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

            // If the currently handled message was the one provided as an argument id, stop there
            if message.id.0 == id_arg {
                end_reached = true;
                break;
            }
        }
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
