use serenity::builder::CreateEmbed;
use serenity::{
    framework::standard::{macros::command, Args, CommandError, CommandResult},
    model::channel::Message,
    prelude::Context,
};

use reqwest::{blocking::get, StatusCode};

// lookup windows cd
// lookup tar

#[command]
pub fn lookup(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut platform = "linux";
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
        _ => return Err(CommandError("Test".to_string())),
    };

    let search_string = args.single::<String>()?;

    let formatted_string = format!(
        "https://raw.githubusercontent.com/tldr-pages/tldr/master/pages/{}/{}.md",
        platform, search_string
    );

    // Search for a page for the given platform and command
    let resp = get(&formatted_string)?;

    match resp.status() {
        StatusCode::NOT_FOUND => (),
        StatusCode::OK => {
            let resp_body = &resp.text()?;

            let (title, description) = match get_tldr_content_from_markdown(resp_body) {
                Some(tuple) => tuple,
                None => return Err(CommandError("Couldn't parse tldr markdown".to_string())),
            };
            let msg = msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(title);
                    e.description(description);
                    e
                });
                m
            });
        }
        s => return Err(CommandError(format!("Unexpected response status: {:?}", s))),
    }

    Ok(())
}

fn get_tldr_content_from_markdown<'m>(tldr_body: &'m str) -> Option<(&'m str, String)> {
    let body_lines: Vec<&str> = tldr_body.lines().collect();

    match body_lines.as_slice() {
        [first_line, rest @ ..] => Some((first_line, rest.join("\n"))),
        _ => None,
    }
}
