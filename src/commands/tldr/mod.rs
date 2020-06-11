use serenity::framework::standard::macros::group;

mod lookup;
use self::lookup::LOOKUP_COMMAND;

#[group]
#[commands(lookup)]
struct Tldr;
