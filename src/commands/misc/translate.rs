use serenity::{
    framework::standard::{macros::command, Args, CommandError, CommandResult},
    model::channel::Message,
    prelude::Context,
};

const LANGUAGES: &[&str] = &["EN", "DE", "EO", "FR", "IT", "JA", "KO", "RU", "ES"];

#[command]
#[description(
    "Translates a given text into the target language given as the first argument. \
        You can optionally prefix the source language as first argument, \
        otherwise it will be auto detected."
)]
#[example("en こんにちは！")]
#[example("de en Guten Abend!")]
pub fn translate(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let first_arg = args.single::<String>().unwrap();
    let second_arg = args.single::<String>().unwrap();

    // Get the target lang (or source lang if second language is given)
    let mut target_lang = match validate_unit(&first_arg) {
        Some(lang) => lang,
        None => {
            return Err(CommandError(
                "The first argument must be a valid target language code!".to_string(),
            ))
        }
    };

    // Try to grab a second language parameter. On success, use that as the target_lang and the
    // initial first parameter as source language (i.e. switch from <target> <text> to <source> <target> <text>)
    let source_lang = match validate_unit(&second_arg) {
        Some(lang) => {
            // When the second argument is a language,
            // swap first and second arguments
            let target_lang_copy = target_lang;
            target_lang = lang;
            target_lang_copy
        }
        None => {
            // Else write this argument back to args, as it's part of the translation string!
            args.rewind();
            "auto".to_string()
        }
    };

    // If nothing is left to translate, a text is missing
    if args.is_empty() {
        return Err(CommandError(
            "There needs to be a text to be translated!".to_string(),
        ));
    }

    let text = args.rest();
    let client = reqwest::blocking::Client::new();

    // Send the query and parse it as text response
    let response = client
        .get(
            format!(
            "https://translate.googleapis.com/translate_a/single?client=gtx&sl={}&tl={}&dt=t&q={}",
            source_lang, target_lang, text
        )
            .as_str(),
        )
        .send()?
        .text()?;

    // Grab translated text from response by grabbing the text between the 1st and 2nd quotation marks
    let quote_indices: Vec<(usize, &str)> = response.match_indices("\"").collect();
    let translation: &str = match quote_indices.len() {
        n if n > 1 => {
            let start = quote_indices[0].0 + 1;
            let end = quote_indices[1].0;
            &response[start..end]
        }
        _ => {
            return Err(CommandError(
                "There was an error parsing the response!".to_string(),
            ))
        }
    };

    // Send message with translation
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(format!(
                "Translation from {} -> {}",
                source_lang, target_lang
            ))
            .description(translation)
        })
    });

    Ok(())
}

fn validate_unit(unit_arg: &String) -> Option<String> {
    let unit_uppercase = unit_arg.to_ascii_uppercase();

    if LANGUAGES.contains(&unit_uppercase.as_str()) {
        Some(unit_uppercase)
    } else {
        None
    }
}
