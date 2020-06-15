use serenity::framework::standard::macros::group;

mod info;

use self::info::INFO_COMMAND;

#[group]
#[commands(info)]
struct System;
