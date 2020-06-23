use serde_json::Value;
use serenity::{
    framework::standard::{macros::command, Args, CommandError, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
#[description(
    "Translates a given text into the target language given as the first argument. \
        You can optionally prefix the source language as first argument, \
        otherwise it will be auto detected."
)]
#[example("en こんにちは！")]
#[example("de en Guten Abend!")]
pub fn translate(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let first_arg = args.single::<String>()?;
    let second_arg = args.single::<String>()?;

    // Get the target lang (or source lang if second language is given)
    let mut target_lang = match validate_unit(&first_arg) {
        Some(lang) => lang,
        None => {
            return Err(CommandError(
                "The first argument must be a valid two letter language code!".to_string(),
            ))
        }
    };

    // Try to grab a second language parameter. On success, use that as the target_lang and the
    // initial first parameter as source language (i.e. switch from <target> <text> to <source> <target> <text>)
    let mut source_lang = match validate_unit(&second_arg) {
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
            "auto"
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

    // Get loosely typed json format
    let json: Value = serde_json::from_str(&response)?;

    // Join translated sentences into one output string
    let data_array = json[0].as_array().unwrap();
    let translated_sentences = data_array
        .iter()
        .fold(String::default(), |translation, data| {
            let transl_sentence = data[0].as_str().unwrap();
            translation + transl_sentence
        });

    // Get recognized source language from response
    source_lang = json[2].as_str().unwrap();

    // Send message with translation
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(format!(
                "Translation from {} -> {}",
                source_lang.to_ascii_uppercase(),
                target_lang.to_ascii_uppercase()
            ))
            .description(translated_sentences)
        })
    });

    Ok(())
}

fn validate_unit(unit_arg: &String) -> Option<&str> {
    let unit_lowercase = unit_arg.to_ascii_lowercase();

    match unit_lowercase.as_str() {
        "Afrikaans" | "af" => Some("af"),
        "Albanian" | "sq" => Some("sq"),
        "Amharic" | "am" => Some("am"),
        "Arabic" | "ar" => Some("ar"),
        "Armenian" | "hy" => Some("hy"),
        "Azerbaijani" | "az" => Some("az"),
        "Basque" | "eu" => Some("eu"),
        "Belarusian" | "be" => Some("be"),
        "Bengali" | "bn" => Some("bn"),
        "Bosnian" | "bs" => Some("bs"),
        "Bulgarian" | "bg" => Some("bg"),
        "Catalan" | "ca" => Some("ca"),
        "Cebuano" | "ceb" => Some("ceb"),
        "Chinese" | "zh-CN" | "zh" => Some("zh"),
        "zh-TW" => Some("zh-TW"),
        "Corsican" | "co" => Some("co"),
        "Croatian" | "hr" => Some("hr"),
        "Czech" | "cs" => Some("cs"),
        "Danish" | "da" => Some("da"),
        "Dutch" | "nl" => Some("nl"),
        "English" | "en" => Some("en"),
        "Esperanto" | "eo" => Some("eo"),
        "Estonian" | "et" => Some("et"),
        "Finnish" | "fi" => Some("fi"),
        "French" | "fr" => Some("fr"),
        "Frisian" | "fy" => Some("fy"),
        "Galician" | "gl" => Some("gl"),
        "Georgian" | "ka" => Some("ka"),
        "German" | "de" => Some("de"),
        "Greek" | "el" => Some("el"),
        "Gujarati" | "gu" => Some("gu"),
        "Haitian" | "Creole" | "ht" => Some("ht"),
        "Hausa" | "ha" => Some("ha"),
        "Hawaiian" | "haw" => Some("haw"),
        "Hebrew" | "he" | "iw" => Some("iw"),
        "Hindi" | "hi" => Some("hi"),
        "Hmong" | "hmn" => Some("hmn"),
        "Hungarian" | "hu" => Some("hu"),
        "Icelandic" | "is" => Some("is"),
        "Igbo" | "ig" => Some("ig"),
        "Indonesian" | "id" => Some("id"),
        "Irish" | "ga" => Some("ga"),
        "Italian" | "it" => Some("it"),
        "Japanese" | "ja" => Some("ja"),
        "Javanese" | "jv" => Some("jv"),
        "Kannada" | "kn" => Some("kn"),
        "Kazakh" | "kk" => Some("kk"),
        "Khmer" | "km" => Some("km"),
        "Kinyarwanda" | "rw" => Some("rw"),
        "Korean" | "ko" => Some("ko"),
        "Kurdish" | "ku" => Some("ku"),
        "Kyrgyz" | "ky" => Some("ky"),
        "Lao" | "lo" => Some("lo"),
        "Latin" | "la" => Some("la"),
        "Latvian" | "lv" => Some("lv"),
        "Lithuanian" | "lt" => Some("lt"),
        "Luxembourgish" | "lb" => Some("lb"),
        "Macedonian" | "mk" => Some("mk"),
        "Malagasy" | "mg" => Some("mg"),
        "Malay" | "ms" => Some("ms"),
        "Malayalam" | "ml" => Some("ml"),
        "Maltese" | "mt" => Some("mt"),
        "Maori" | "mi" => Some("mi"),
        "Marathi" | "mr" => Some("mr"),
        "Mongolian" | "mn" => Some("mn"),
        "Myanmar" | "my" => Some("my"),
        "Nepali" | "ne" => Some("ne"),
        "Norwegian" | "no" => Some("no"),
        "Nyanja" | "ny" => Some("ny"),
        "Odia" | "or" => Some("or"),
        "Pashto" | "ps" => Some("ps"),
        "Persian" | "fa" => Some("fa"),
        "Polish" | "pl" => Some("pl"),
        "Portuguese" | "pt" => Some("pt"),
        "Punjabi" | "pa" => Some("pa"),
        "Romanian" | "ro" => Some("ro"),
        "Russian" | "ru" => Some("ru"),
        "Samoan" | "sm" => Some("sm"),
        "Scots" | "Gaelic" | "gd" => Some("gd"),
        "Serbian" | "sr" => Some("sr"),
        "Sesotho" | "st" => Some("st"),
        "Shona" | "sn" => Some("sn"),
        "Sindhi" | "sd" => Some("sd"),
        "Sinhala" | "si" => Some("si"),
        "Slovak" | "sk" => Some("sk"),
        "Slovenian" | "sl" => Some("sl"),
        "Somali" | "so" => Some("so"),
        "Spanish" | "es" => Some("es"),
        "Sundanese" | "su" => Some("su"),
        "Swahili" | "sw" => Some("sw"),
        "Swedish" | "sv" => Some("sv"),
        "Tagalog" | "tl" => Some("tl"),
        "Tajik" | "tg" => Some("tg"),
        "Tamil" | "ta" => Some("ta"),
        "Tatar" | "tt" => Some("tt"),
        "Telugu" | "te" => Some("te"),
        "Thai" | "th" => Some("th"),
        "Turkish" | "tr" => Some("tr"),
        "Turkmen" | "tk" => Some("tk"),
        "Ukrainian" | "uk" => Some("uk"),
        "Urdu" | "ur" => Some("ur"),
        "Uyghur" | "ug" => Some("ug"),
        "Uzbek" | "uz" => Some("uz"),
        "Vietnamese" | "vi" => Some("vi"),
        "Welsh" | "cy" => Some("cy"),
        "Xhosa" | "xh" => Some("xh"),
        "Yiddish" | "yi" => Some("yi"),
        "Yoruba" | "yo" => Some("yo"),
        "Zulu" | "zu" => Some("zu"),
        _ => None,
    }
}
