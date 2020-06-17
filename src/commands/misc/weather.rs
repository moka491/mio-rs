use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, Utc};
use serde::Deserialize;
use serenity::{
    framework::standard::{macros::command, Args, CommandError, CommandResult},
    model::channel::Message,
    prelude::Context,
};
use std::env;

#[command]
#[description("Retrieves the weather forecast at the given location")]
#[example("Berlin")]
#[example("Sri Lanka")]
#[example("New York")]
pub fn weather(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let token = match env::var("OPEN_WEATHER_MAP_TOKEN") {
        Ok(token) => token,
        Err(e) => {
            return Err(CommandError(
                "Couldn't load api key from config".to_string(),
            ))
        }
    };

    let client = reqwest::blocking::Client::new();

    // Get coordinates for given location
    let search_arg = args.single::<String>()?;

    let location: LocationQueryResponse = client
        .get("http://api.openweathermap.org/data/2.5/weather")
        .query(&[("appid", &token), ("q", &search_arg)])
        .send()?
        .json()?;

    let weather: WeatherQueryResponse = client
        .get("http://api.openweathermap.org/data/2.5/onecall")
        .query(&[
            ("appid", &token),
            ("lat", &location.coord.lat.to_string()),
            ("lon", &location.coord.lon.to_string()),
            ("units", &"metric".to_string()),
        ])
        .send()?
        .json()?;

    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(format!("Weather in {}", search_arg))
                .thumbnail(get_weather_image_url(&weather.current.weather[0].icon))
                .description(format!(
                    "**{}** \n\
                    Temp: **{}**°C (Feels like **{}**°C)",
                    uppercase_first(&weather.current.weather[0].description),
                    &weather.current.temp,
                    &weather.current.feels_like
                ))
                .fields(vec![
                    (
                        "Weather",
                        format!(
                            "**Clouds**: {}% \n\
                            **Humidity**: {}% \n\
                            **Pressure**: {} hpa",
                            &weather.current.clouds,
                            &weather.current.humidity,
                            &weather.current.pressure
                        ),
                        true,
                    ),
                    (
                        "Wind",
                        format!(
                            "**Speed**: {}\n\
                            **Direction**: {}° ({})",
                            &weather.current.wind_speed,
                            weather.current.wind_deg,
                            format_direction(weather.current.wind_deg)
                        ),
                        true,
                    ),
                    (
                        "Location",
                        format!(
                            "**Sunrise**: {}\n\
                            **Sunset**: {}\n\
                            **Local Time**: {}",
                            format_timestamp(
                                weather.current.sunrise,
                                weather.timezone_offset,
                                false
                            ),
                            format_timestamp(
                                weather.current.sunset,
                                weather.timezone_offset,
                                false
                            ),
                            format_timestamp(weather.current.dt, weather.timezone_offset, true),
                        ),
                        false,
                    ),
                ])
        })
    });

    Ok(())
}

fn get_weather_image_url(code: &String) -> String {
    format!("http://openweathermap.org/img/wn/{}@2x.png", code)
}

fn uppercase_first(s: &str) -> String {
    format!("{}{}", (&s[..1].to_string()).to_uppercase(), &s[1..])
}

fn format_timestamp(timestamp: i64, offset: i32, show_date: bool) -> String {
    let date_time = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);
    let local_time = date_time.with_timezone(&FixedOffset::east(offset));

    if show_date {
        local_time.format("%H:%M, %e %b %Y").to_string()
    } else {
        local_time.format("%H:%M").to_string()
    }
}

fn format_direction(degrees: i32) -> String {
    match degrees {
        x if x <= 45 => "North".to_string(),
        x if x <= 135 => "East".to_string(),
        x if x <= 225 => "South".to_string(),
        x if x <= 315 => "West".to_string(),
        _ => "".to_string(),
    }
}

#[derive(Deserialize, Debug)]
struct LocationQueryResponse {
    coord: Location,
    // Skip all the other data
}
#[derive(Deserialize, Debug)]
struct Location {
    lon: f64,
    lat: f64,
}

#[derive(Deserialize, Debug)]
struct WeatherQueryResponse {
    timezone_offset: i32,
    current: CurrentWeather,
    daily: Vec<DailyWeather>,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    dt: i64,
    sunrise: i64,
    sunset: i64,
    temp: f64,
    feels_like: f64,
    pressure: i32,
    humidity: i32,
    clouds: i32,
    visibility: i32,
    wind_speed: f64,
    wind_deg: i32,
    weather: Vec<Weather>,
}
#[derive(Deserialize, Debug)]
struct DailyWeather {
    temp: Temp,
    feels_like: FeelsLike,
    pressure: i32,
    humidity: i32,
    wind_speed: f64,
    wind_deg: i32,
    weather: Vec<Weather>,
}
#[derive(Deserialize, Debug)]
struct Weather {
    id: i32,
    main: String,
    description: String,
    icon: String,
}
#[derive(Deserialize, Debug)]
struct Temp {
    day: f64,
    min: f64,
    max: f64,
    night: f64,
    eve: f64,
    morn: f64,
}
#[derive(Deserialize, Debug)]
struct FeelsLike {
    day: f64,
    night: f64,
    eve: f64,
    morn: f64,
}
