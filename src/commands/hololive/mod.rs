use serenity::framework::standard::macros::group;

mod schedule;
use self::schedule::SCHEDULE_COMMAND;

#[group]
#[prefixes("hololive", "hl")]
#[commands(schedule)]
struct Hololive;
