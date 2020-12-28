use serenity::framework::standard::macros::group;

mod nhentai;

use self::nhentai::NHENTAI_COMMAND;

#[group]
#[commands(nhentai)]
struct NSFW;
