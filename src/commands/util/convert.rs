use serenity::{
    framework::standard::{macros::command, Args, CommandError, CommandResult},
    model::channel::Message,
    prelude::Context,
};

enum Velocity {
    KILOMETERSPERHOUR,
    MILESPERHOUR,
    METERSPERSECOND,
    FEETPERSECOND,
}

enum Distance {
    KILOMETER,
    METER,
    CENTIMETER,
    MILLIMETER,
    MILE,
    FOOT,
    YARD,
    INCH,
}

enum Temperature {
    CELSIUS,
    KELVIN,
    FAHRENHEIT,
}

static velocity_matrix: [[fn(f64) -> f64; 4]; 4] = [
    //  kilometers per hour
    [
        |n| n,          // to kilometers per hour
        |n| n * 0.62,   // to miles per hour
        |n| n * 0.2778, // to meters per second
        |n| n * 0.9113, // to feet per second
    ],
    // miles per hour
    [
        |n| n * 1.609, // to kilometers per hour
        |n| n,         // to miles per hour
        |n| n * 0.447, // to meters per second
        |n| n * 1.467, // to feet per second
    ],
    // meters per second
    [
        |n| n * 3.6,   // to kilometers per hour
        |n| n * 2.237, // to miles per hour
        |n| n,         // to meters per second
        |n| n * 3.281, // to feet per second
    ],
    // feet per second
    [
        |n| n * 1.097,  // to kilometers per hour
        |n| n * 0.6818, // to miles per hour
        |n| n * 0.3048, // to meters per second
        |n| n,          // to feet per second
    ],
];

static distance_matrix: [[fn(f64) -> f64; 8]; 8] = [
    // kilometer
    [
        |n| n,             // to kilometer
        |n| n * 1000.0,    // to meter
        |n| n * 100000.0,  // to centimeter
        |n| n * 1000000.0, // to millimeter
        |n| n * 0.6214,    // to mile
        |n| n * 3281.0,    // to foot
        |n| n * 1094.0,    // to yard
        |n| n * 39370.0,   // to inch
    ],
    // meter
    [
        |n| n * 0.001,     // to kilometer
        |n| n,             // to meter
        |n| n * 100.0,     // to centimeter
        |n| n * 1000.0,    // to millimeter
        |n| n * 0.0006214, // to mile
        |n| n * 3.281,     // to foot
        |n| n * 1.094,     // to yard
        |n| n * 39.370,    // to inch
    ],
    // centimeter
    [
        |n| n * 0.00001,     // to kilometer
        |n| n * 0.01,        // to meter
        |n| n,               // to centimeter
        |n| n * 10.0,        // to millimeter
        |n| n * 0.000006214, // to mile
        |n| n * 0.03281,     // to foot
        |n| n * 0.01094,     // to yard
        |n| n * 0.3937,      // to inch
    ],
    // millimeter
    [
        |n| n * 0.000001,     // to kilometer
        |n| n * 0.001,        // to meter
        |n| n * 0.1,          // to centimeter
        |n| n,                // to millimeter
        |n| n * 0.0000006214, // to mile
        |n| n * 0.003281,     // to foot
        |n| n * 0.001094,     // to yard
        |n| n * 0.03937,      // to inch
    ],
    // mile
    [
        |n| n * 1.609,     // to kilometer
        |n| n * 1609.0,    // to meter
        |n| n * 160900.0,  // to centimeter
        |n| n * 1609000.0, // to millimeter
        |n| n,             // to mile
        |n| n * 5280.0,    // to foot
        |n| n * 1760.0,    // to yard
        |n| n * 63360.0,   // to inch
    ],
    // foot
    [
        |n| n * 0.0003048, // to kilometer
        |n| n * 0.3048,    // to meter
        |n| n * 30.48,     // to centimeter
        |n| n * 304.8,     // to millimeter
        |n| n * 0.0001894, // to mile
        |n| n,             // to foot
        |n| n / 3.0,       // to yard
        |n| n * 12.0,      // to inch
    ],
    // yard
    [
        |n| n * 0.0009144, // to kilometer
        |n| n * 0.9144,    // to meter
        |n| n * 91.44,     // to centimeter
        |n| n * 914.4,     // to millimeter
        |n| n * 0.0005682, // to mile
        |n| n * 3.0,       // to foot
        |n| n,             // to yard
        |n| n * 36.0,      // to inch
    ],
    // inch
    [
        |n| n * 0.0000254,  // to kilometer
        |n| n * 0.0254,     // to meter
        |n| n * 2.54,       // to centimeter
        |n| n * 25.4,       // to millimeter
        |n| n * 0.00001578, // to mile
        |n| n / 12.0,       // to foot
        |n| n / 36.0,       // to yard
        |n| n,              // to inch
    ],
];

static temp_matrix: [[fn(f64) -> f64; 3]; 3] = [
    // celcius
    [
        |n| n,              // to celcius
        |n| n - 274.2,      // to kelvin
        |n| n * 1.8 + 32.0, // to fahrenheit
    ],
    // kelvin
    [
        |n| n + 274.2,        // to celcius
        |n| n,                // to kelvin
        |n| n * 1.8 - 459.67, // to fahrenheit
    ],
    // fahrenheit
    [
        |n| (n - 32.0) * 1.8,   // to celcius,
        |n| (n + 459.67) * 1.8, // to kelvin
        |n| n,                  // to fahrenheit
    ],
];

#[command]
pub fn convert(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() != 2 {
        return Err(CommandError("Invalid number of arguments. You need to pass two numbers with the source unit and destined unit.".to_string()));
    }

    let source_arg = args.single::<String>()?;
    let dest_unit_arg = args.single::<String>()?;

    // ~convert 2.5 km/s f
    // ~convert 25ms kmh

    let source_number_str = source_arg.trim_end_matches(|c: char| !c.is_numeric());
    let source_unit_str = source_arg.trim_start_matches(source_number_str);

    let source_number = source_number_str.parse::<f64>()?;

    let source_unit = match get_unit(source_unit_str) {
        Some(unit) => unit,
        None => return Err(CommandError("Invalid source unit.".to_string())),
    };

    let dest_unit = match get_unit(dest_unit_arg.as_str()) {
        Some(unit) => unit,
        None => return Err(CommandError("Invalid destination unit.".to_string())),
    };

    let result = calc_conversion(source_number, source_unit, dest_unit);

    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.content(format!("Number: {}, Unit: {}", source_number, source_unit));
        m
    });

    Ok(())
}

fn get_unit(unit_string: &str) -> Option<Unit> {
    match unit_string.to_lowercase().as_str() {
        "kmh" | "km/h" => Some(Unit::KILOMETERSPERHOUR),
        "ms" | "m/s" => Some(Unit::METERSPERSECOND),
        "mph" | "m/h" => Some(Unit::MILESPERHOUR),

        "km" => Some(Unit::KILOMETER),
        "m" => Some(Unit::METER),
        "cm" => Some(Unit::CENTIMETER),
        "mm" => Some(Unit::MILLIMETER),
        "mi" => Some(Unit::MILE),
        "ft" => Some(Unit::FOOT),
        "yd" => Some(Unit::YARD),

        "c" | "°c" => Some(Unit::CELSIUS),
        "k" => Some(Unit::KELVIN),
        "f" | "°f" => Some(Unit::FAHRENHEIT),

        _ => None,
    }
}

fn calc_conversion(number: f64, source_unit: Unit, dest_unit: Unit) -> Option<f64> {
    match (source_unit, dest_unit) {
        (Unit::KILOMETERSPERHOUR, Unit::MILESPERHOUR) => Some(number * 0.62),
        (Unit::MILESPERHOUR, Unit::KILOMETERSPERHOUR) => Some(number * 1.61),
        _ => None,
    }
}
