use serenity::framework::standard::macros::group;

mod convert;
mod fetch;
mod say;
mod translate;
mod weather;

use self::convert::CONVERT_COMMAND;
use self::fetch::FETCH_COMMAND;
use self::say::SAY_COMMAND;
use self::say::YELL_COMMAND;
use self::translate::TRANSLATE_COMMAND;
use self::weather::WEATHER_COMMAND;

#[group]
#[commands(convert, say, yell, weather, translate, fetch)]
struct Misc;
