use serenity::framework::standard::macros::group;

pub mod schedule;

use self::schedule::SCHEDULE_COMMAND;

#[group]
#[commands(schedule)]
struct Hololive;
