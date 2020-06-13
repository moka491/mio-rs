use lazy_static::lazy_static;
use serenity::{
    framework::standard::{macros::command, Args, CommandError, CommandResult},
    model::channel::Message,
    prelude::Context,
};

enum Unit {
    Velocity(VelocityType),
    Distance(DistanceType),
    Temperature(TemperatureType),
}

enum VelocityType {
    KILOMETERSPERHOUR,
    MILESPERHOUR,
    METERSPERSECOND,
    FEETPERSECOND,
}

enum DistanceType {
    KILOMETER,
    METER,
    CENTIMETER,
    MILLIMETER,
    MILE,
    FOOT,
    YARD,
    INCH,
}

enum TemperatureType {
    CELSIUS,
    KELVIN,
    FAHRENHEIT,
}

lazy_static! {
    static ref VELOCITY_MATRIX: Vec<Vec<fn(f64) -> f64>> = vec![
        //  kilometers per hour
        vec![
            |n| n,          // to kilometers per hour
            |n| n * 0.62,   // to miles per hour
            |n| n * 0.2778, // to meters per second
            |n| n * 0.9113, // to feet per second
        ],
        // miles per hour
        vec![
            |n| n * 1.609, // to kilometers per hour
            |n| n,         // to miles per hour
            |n| n * 0.447, // to meters per second
            |n| n * 1.467, // to feet per second
        ],
        // meters per second
        vec![
            |n| n * 3.6,   // to kilometers per hour
            |n| n * 2.237, // to miles per hour
            |n| n,         // to meters per second
            |n| n * 3.281, // to feet per second
        ],
        // feet per second
        vec![
            |n| n * 1.097,  // to kilometers per hour
            |n| n * 0.6818, // to miles per hour
            |n| n * 0.3048, // to meters per second
            |n| n,          // to feet per second
        ],
    ];

    static ref DISTANCE_MATRIX: Vec<Vec<fn(f64) -> f64>> = vec![
        // kilometer
        vec![
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
        vec![
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
        vec![
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
        vec![
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
        vec![
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
        vec![
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
        vec![
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
        vec![
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

    static ref TEMP_MATRIX: Vec<Vec<fn(f64) -> f64>> = vec![
        // celcius
        vec![
            |n| n,              // to celcius
            |n| n - 274.2,      // to kelvin
            |n| n * 1.8 + 32.0, // to fahrenheit
        ],
        // kelvin
        vec![
            |n| n + 274.2,        // to celcius
            |n| n,                // to kelvin
            |n| n * 1.8 - 459.67, // to fahrenheit
        ],
        // fahrenheit
        vec![
            |n| (n - 32.0) * 1.8,   // to celcius,
            |n| (n + 459.67) * 1.8, // to kelvin
            |n| n,                  // to fahrenheit
        ],
    ];
}

#[command]
pub fn convert(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() != 2 {
        return Err(CommandError(
            "Invalid number of arguments. You need to pass <number><unit> and <desired unit>, e.g. ~convert 25km/s ft/s".to_string(),
        ));
    }

    let source_arg = args.single::<String>()?;
    let dest_unit_arg = args.single::<String>()?;

    let source_number_str = source_arg.trim_end_matches(|c: char| !c.is_numeric());
    let source_unit_str = source_arg.trim_start_matches(source_number_str);

    let source_number = source_number_str.parse::<f64>()?;

    // 1. check if source and destination are valid units and match some variant
    // 2. check if source and destination variants are of the same type
    // 3. resolve units to indices and

    let source_unit = match get_unit(source_unit_str) {
        Some(unit) => unit,
        None => return Err(CommandError("Invalid source unit.".to_string())),
    };

    let dest_unit = match get_unit(dest_unit_arg.as_str()) {
        Some(unit) => unit,
        None => return Err(CommandError("Invalid destination unit.".to_string())),
    };

    // Check if both source and destination unit are of the same type (distance, velocity etc),
    // and then do the conversation using the appropriate matrix and the indices dictated by the unit t1/t2
    let result = match (source_unit, dest_unit) {
        (Unit::Velocity(t1), Unit::Velocity(t2)) => {
            do_conversion(&VELOCITY_MATRIX, source_number, t1 as usize, t2 as usize)
        }
        (Unit::Distance(t1), Unit::Distance(t2)) => {
            do_conversion(&DISTANCE_MATRIX, source_number, t1 as usize, t2 as usize)
        }
        (Unit::Temperature(t1), Unit::Temperature(t2)) => {
            do_conversion(&TEMP_MATRIX, source_number, t1 as usize, t2 as usize)
        }
        _ => {
            return Err(CommandError(
                "Can't convert between unrelated units.".to_string(),
            ))
        }
    };

    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.content(format!(
            "{} -> {} = {:.2}{1}",
            input = source_arg,
            dest_unit = dest_unit_arg,
            result = result
        ));
        m
    });

    Ok(())
}

fn get_unit(unit_string: &str) -> Option<Unit> {
    match unit_string.to_lowercase().as_str() {
        "kmh" | "km/h" => Some(Unit::Velocity(VelocityType::KILOMETERSPERHOUR)),
        "ms" | "m/s" => Some(Unit::Velocity(VelocityType::METERSPERSECOND)),
        "mph" | "m/h" => Some(Unit::Velocity(VelocityType::MILESPERHOUR)),
        "fts" | "ft/s" => Some(Unit::Velocity(VelocityType::FEETPERSECOND)),

        "km" => Some(Unit::Distance(DistanceType::KILOMETER)),
        "m" => Some(Unit::Distance(DistanceType::METER)),
        "cm" => Some(Unit::Distance(DistanceType::CENTIMETER)),
        "mm" => Some(Unit::Distance(DistanceType::MILLIMETER)),
        "mi" => Some(Unit::Distance(DistanceType::MILE)),
        "ft" => Some(Unit::Distance(DistanceType::FOOT)),
        "yd" => Some(Unit::Distance(DistanceType::YARD)),
        "in" | "inch" | "inches" => Some(Unit::Distance(DistanceType::INCH)),

        "c" | "°c" => Some(Unit::Temperature(TemperatureType::CELSIUS)),
        "k" => Some(Unit::Temperature(TemperatureType::KELVIN)),
        "f" | "°f" => Some(Unit::Temperature(TemperatureType::FAHRENHEIT)),

        _ => None,
    }
}

fn do_conversion(
    matrix: &Vec<Vec<fn(f64) -> f64>>,
    number: f64,
    matrix_source_index: usize,
    matrix_dest_index: usize,
) -> f64 {
    matrix[matrix_source_index][matrix_dest_index](number)
}
