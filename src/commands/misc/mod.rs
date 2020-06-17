use serenity::framework::standard::macros::group;

mod convert;
mod say;
mod weather;

use self::convert::CONVERT_COMMAND;
use self::say::SAY_COMMAND;
use self::say::YELL_COMMAND;
use self::weather::WEATHER_COMMAND;

#[group]
#[commands(convert, say, yell, weather)]
struct Misc;
