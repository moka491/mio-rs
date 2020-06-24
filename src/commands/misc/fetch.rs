use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    http::AttachmentType,
    model::{channel::Message, id::MessageId},
    prelude::Context,
};
use std::borrow::Cow;

const MAX_MESSAGE_COUNT: u64 = 5000;
const REQUESTS_PER_ITER: u64 = 100;

#[command]
#[description("Generate a list of all the images posted recently")]
pub fn fetch(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut link_list: Vec<String> = vec![];

    let mut last_message_id: MessageId = msg.id;
    let mut message_counter: u64 = 0;
    let mut end_reached = false;

    let id_arg = args.single::<u64>().unwrap_or_default();

    while !end_reached {
        // Show typing status
        let _ = msg.channel_id.broadcast_typing(&ctx.http);

        // Fetch REQUESTS_PER_ITER messages to process
        let _messages: Vec<Message> = msg.channel_id.messages(&ctx.http, |retriever| {
            retriever.before(last_message_id).limit(REQUESTS_PER_ITER)
        })?;

        // Count up how many messages have been requested so far
        message_counter += _messages.len() as u64;
        last_message_id = _messages.last().unwrap().id;

        // If the retrieved messages are less than what expected (usually means we reached the beginning of the history)
        // or we reached a max amount of requests to make, stop after this iteration
        if _messages.len() < REQUESTS_PER_ITER as usize || message_counter >= MAX_MESSAGE_COUNT {
            end_reached = true;
        }

        // Go through all fetched messages in this iteration
        for message in _messages {
            // Gather all image attachments and add their urls to the list
            message
                .attachments
                .iter()
                .filter(|a| a.width.is_some())
                .for_each(|a| link_list.push(a.url.clone()));

            // Gather all embedded images and add their urls to the list
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

        // Create attachment text file
        let attachment: AttachmentType = AttachmentType::Bytes {
            data: Cow::from(file_bytes),
            filename: "found_images.txt".to_string(),
        };

        // Send result info message
        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Image fetching results")
                .description(format!(
                    "Found **{}** images in **{}** messages! \n\
                    The last message processed was [this one](https://discord.com/channels/{}/{}/{}/). \n\
                    \n\
                    You can download the attached txt file and \n\
                    import it into a download manager of your choice."
                , link_list.len(), message_counter, msg.guild_id.unwrap(), msg.channel_id.0, last_message_id.0 ))
            })
        });

        // Send actual attachment
        let _ = msg
            .channel_id
            .send_message(&ctx.http, |m| m.add_file(attachment));
    }

    Ok(())
}
