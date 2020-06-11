use serenity::framework::standard::macros::group;

mod lookup;
use self::lookup::TLDR_COMMAND;

#[group]
#[commands(tldr)]
struct Tldr;
