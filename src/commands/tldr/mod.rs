use serenity::framework::standard::macros::group;

mod lookup;

use self::lookup::LOOKUP_COMMAND;

#[group]
#[prefixes("tldr")]
#[default_command(lookup)]
#[commands(lookup)]
struct Tldr;
