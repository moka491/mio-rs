use log::debug;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    http::AttachmentType,
    model::{channel::Message, id::MessageId},
    prelude::Context,
};
use std::borrow::Cow;

const REQUESTS_PER_ITER: u64 = 100;
const MESSAGE_RELATIVE_AGE_THRESH: i64 = 3600 * 18;
const MESSAGE_NO_IMAGES_FOUND_THRESH: u64 = 50;

#[command]
#[description("Generate a list of all the images recently posted. It will try to intelligently guess where the image posting stopped, but you can also define a clear start and/or end point")]
#[usage("<optional starting Message ID> <optional ending message ID>")]
#[example("725681148134424596")]
#[example("725681148134424596 725681148134424582")]
pub async fn fetch(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut link_list: Vec<String> = vec![];

    // Get optional start and end parameters ("to" message older than "from" message)
    // If the to id is not given, leave it at 0 (thus it won't match an end)
    // If the from id is not given, start at this command's message
    let to_msg_id_arg = args.single::<u64>().unwrap_or_default();
    let from_msg_id_arg = args.single::<u64>().unwrap_or(msg.id.0);

    // Search starts either at current bot message or at the given start id
    let mut last_message_id: MessageId = MessageId(from_msg_id_arg);

    // Store last processed message timestamp
    let mut current_message_timestamp: i64;
    let mut last_message_timestamp: i64 = 0;

    let mut message_processed_counter: u64 = 0;
    let mut message_nothing_found_counter: u64 = 0;

    let mut end_reached = false;
    let end_point_defined = to_msg_id_arg > 0;

    while !end_reached {
        // Show typing status
        let _ = msg.channel_id.broadcast_typing(&ctx.http);

        // Fetch REQUESTS_PER_ITER messages to process
        let _messages: Vec<Message> = msg
            .channel_id
            .messages(&ctx.http, |retriever| {
                retriever.before(last_message_id).limit(REQUESTS_PER_ITER)
            })
            .await?;

        debug!("Requested {} new messages from discord", &REQUESTS_PER_ITER);

        // If the retrieved messages are less than what expected (usually means we reached the beginning of the history)
        // or we reached a max amount of requests to make, stop after this iteration
        if _messages.len() < REQUESTS_PER_ITER as usize {
            end_reached = true;
        }

        // Go through all fetched messages in this iteration
        for message in _messages {
            debug!("Processing message {}", message.id.0);

            current_message_timestamp = message.timestamp.timestamp();

            // Checks before working on current message:
            // If no clear end point was given as an argument,
            // Stop searching based on if the current message is significantly older than the last one (relative age threshold)
            if !end_point_defined
                && last_message_timestamp - current_message_timestamp >= MESSAGE_RELATIVE_AGE_THRESH
            {
                debug!(
                    "Stopped due to the current message being {} seconds older than the last one",
                    last_message_timestamp - current_message_timestamp
                );

                end_reached = true;
                break;
            }

            // If no clear end point was given as an argument,
            // Stop searching based on if any of the last messages even had images (no images found threshold)
            if !end_point_defined && message_nothing_found_counter >= MESSAGE_NO_IMAGES_FOUND_THRESH
            {
                debug!(
                    "Stopped since there's been no images for the last {} messages now",
                    message_nothing_found_counter
                );

                end_reached = true;
                break;
            }

            // Gather all image attachments
            let mut attachment_filter = message
                .attachments
                .iter()
                .filter(|a| a.width.is_some())
                .peekable();

            // Gather all embedded images
            let mut embed_filter = message
                .embeds
                .iter()
                .filter_map(|e| e.image.clone())
                .peekable();

            // If there's either attachments or embeds containing at least one image,
            // then add all of them to the link list and reset the "nothing found" counter to 0.
            // Otherwise increment the counter
            if attachment_filter.peek().is_some() || embed_filter.peek().is_some() {
                message_nothing_found_counter = 0;

                attachment_filter.for_each(|a| link_list.push(a.url.clone()));
                embed_filter.for_each(|e| link_list.push(e.url.clone()));
            } else {
                message_nothing_found_counter += 1;

                debug!("No images found in this message");
            }

            // Checks after working on the current message
            // If the currently handled message was the one provided as an argument for the end, stop there
            if message.id.0 == to_msg_id_arg {
                end_reached = true;
                break;
            }

            // Update iteration stats
            last_message_timestamp = current_message_timestamp;
            last_message_id = message.id;
            message_processed_counter += 1;
        }
    }

    // Send results and the link list when images have been found
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
                    "Found **{}** images in processed **{}** messages! \n\
                    The last message processed was [this one](https://discord.com/channels/{}/{}/{}/). \n\
                    \n\
                    You can download the attached txt file and \n\
                    import it into a download manager of your choice."
                , link_list.len(), message_processed_counter, msg.guild_id.unwrap(), msg.channel_id.0, last_message_id.0 ))
            })
        }).await;

        // Send actual attachment
        let _ = msg
            .channel_id
            .send_message(&ctx.http, |m| m.add_file(attachment))
            .await;

    // If not, inform the user that nothing's been found
    } else {
        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Image fetching results")
                .description(format!(
                    "Processed **{}** messages but I haven't found any images :( \n\
                    The last message processed was [this one](https://discord.com/channels/{}/{}/{}/)."
                , message_processed_counter, msg.guild_id.unwrap(), msg.channel_id.0, last_message_id.0 ))
            })
        }).await;
    }

    Ok(())
}
