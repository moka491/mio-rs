use serenity::framework::standard::macros::group;

mod convert;
mod say;

use self::convert::CONVERT_COMMAND;
use self::say::SAY_COMMAND;

#[group]
#[commands(convert, say)]
struct Misc;
