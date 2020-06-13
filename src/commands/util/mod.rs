use serenity::framework::standard::macros::group;

mod convert;
use self::convert::CONVERT_COMMAND;

#[group]
#[commands(convert)]
struct Util;
