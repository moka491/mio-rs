use crate::core::consts::MAIN_COLOR;
use serenity::{
    framework::standard::{macros::command, Args, CommandError, CommandResult},
    model::channel::Message,
    prelude::Context,
    utils::Colour,
};

use reqwest::{get, StatusCode};

#[command]
pub async fn lookup(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let platform_arg;

    // If the platform was given as the first argument, try to parse it, otherwise go back to linux
    let platform = match args.len() {
        1 => "linux",
        2 => {
            platform_arg = args.single::<String>().unwrap();

            if let "linux" | "windows" | "macos" = platform_arg.as_str() {
                platform_arg.as_str()
            } else {
                "linux"
            }
        }
        _ => return Err(CommandError::from("Invalid number of arguments")),
    };

    // Retrieve the command search string argument
    let search_string = args.single::<String>().unwrap();

    let tldr_urls = [
        format!(
            "https://raw.githubusercontent.com/tldr-pages/tldr/master/pages/{}/{}.md",
            platform, search_string
        ),
        format!(
            "https://raw.githubusercontent.com/tldr-pages/tldr/master/pages/common/{}.md",
            search_string
        ),
    ];

    // Try to find the search string on any url
    for url in tldr_urls.iter() {
        let resp = get(url).await.unwrap();

        match resp.status() {
            // If the file is not found on the current url, try the next one
            StatusCode::NOT_FOUND => (),
            // If it was found, parse it, send the embed and Ok() out
            StatusCode::OK => {
                let resp_body = &resp.text().await.unwrap();

                let (title, description) = match get_tldr_content_from_markdown(resp_body) {
                    Some(tuple) => tuple,
                    None => return Err(CommandError::from("Couldn't parse tldr markdown")),
                };

                let _ = msg
                    .channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.colour(Colour::new(MAIN_COLOR))
                                .title(title)
                                .description(description)
                        })
                    })
                    .await;

                return Ok(());
            }
            // On any other response, throw an error
            s => {
                return Err(CommandError::from(format!(
                    "Unexpected response status: {:?}",
                    s
                )))
            }
        }
    }

    // Send a message if nothing was found until now
    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content(format!(
                "Could not find a tl:dr page for '{}'",
                search_string
            ))
        })
        .await;

    Ok(())
}

fn get_tldr_content_from_markdown(tldr_body: &str) -> Option<(&str, String)> {
    let body_lines: Vec<&str> = tldr_body.lines().collect();

    match body_lines.as_slice() {
        [first_line, rest @ ..] => {
            // Remove any hashtags and spaces at the start of the title line
            let heading = first_line.trim_start_matches(|c| c == ' ' || c == '#');
            Some((heading, rest.join("\n")))
        }
        _ => None,
    }
}
