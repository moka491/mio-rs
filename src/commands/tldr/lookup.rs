use serenity::{
    framework::standard::{macros::command, Args, CommandError, CommandResult},
    model::channel::Message,
    prelude::Context,
};

use reqwest::{blocking::get, StatusCode};

#[command]
pub fn lookup(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let platform_arg;

    // If the platform was given as the first argument, try to parse it, otherwise go back to linux
    let platform = match args.len() {
        1 => "linux",
        2 => {
            platform_arg = args.single::<String>()?;

            if let "linux" | "windows" | "macos" = platform_arg.as_str() {
                platform_arg.as_str()
            } else {
                "linux"
            }
        }
        _ => return Err(CommandError("Invalid number of arguments".to_string())),
    };

    // Retrieve the command search string argument
    let search_string = args.single::<String>()?;

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
        let resp = get(url)?;

        match resp.status() {
            // If the file is not found on the current url, try the next one
            StatusCode::NOT_FOUND => (),
            // If it was found, parse it, send the embed and Ok() out
            StatusCode::OK => {
                let resp_body = &resp.text()?;

                let (title, description) = match get_tldr_content_from_markdown(resp_body) {
                    Some(tuple) => tuple,
                    None => return Err(CommandError("Couldn't parse tldr markdown".to_string())),
                };

                let _ = msg.channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title(title);
                        e.description(description);
                        e
                    });
                    m
                });

                return Ok(());
            }
            // On any other response, throw an error
            s => return Err(CommandError(format!("Unexpected response status: {:?}", s))),
        }
    }

    // Send a message if nothing was found until now
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.content(format!(
            "Could not find a tl:dr page for '{}'",
            search_string
        ));
        m
    });

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
