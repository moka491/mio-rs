use serenity::{
    framework::standard::{macros::command, Args, CommandError, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
pub fn convert(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() != 2 {
        return Err(CommandError("Invalid number of arguments. You need to pass two numbers with the source unit and destined unit.".to_string()));
    }

    let source_arg = args.single::<String>()?;
    let dest_unit = args.single::<String>()?;

    // ~convert 2.5 km/s f
    // ~convert 25ms kmh

    let source_number = source_arg.trim_end_matches(|c: char| !c.is_numeric());
    let source_unit = source_arg.trim_start_matches(source_number);

    let result = match (source_unit, dest_unit) {
        ("kmh", "mph") => kmh_to_mph(source_number),
        ("mph", "kmh") => mph_to_kmh(source_number),
        _ => return Err(CommandError("Invalid conversion units.".to_string())),
    };

    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.content(format!("Number: {}, Unit: {}", source_number, source_unit));
        m
    });

    Ok(())
}
