extern crate rustc_version_runtime;
extern crate sys_info;
use rustc_version_runtime::version;
use sys_info::*;

use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

const BOT_VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[command]
pub fn info(ctx: &mut Context, msg: &Message, _: Args) -> CommandResult {
    let app_info = &ctx.http.get_current_application_info()?;
    let bot_user = &ctx.cache.read().user;
    let bot_avatar = bot_user.avatar_url().unwrap_or(String::from(""));

    let load = loadavg().unwrap();
    let mem = mem_info().unwrap();

    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("About Ookami Mio")
                .description(
                    "**こんばんみぉーん**！ I'm Ookami Mio, your fellow helper bot written in [Rust](https://www.rust-lang.org/) using [serenity](https://github.com/serenity-rs/serenity)",
                )
                .thumbnail(bot_avatar)
                .fields(vec![
                    ("Bot Info", format!("  **Version**: v{}\n\
                                            **Compiled with**: rustc v{}\n\
                                            **Owner**: {}#{:04}", BOT_VERSION, version(), app_info.owner.name, app_info.owner.discriminator), true),
                    ("Dependencies", "  **serenity-rs**: v0.8.6\n\
                                        **reqwest**: v0.10".to_string(), true),
                    ("System Info", format!("   **OS**: {} {}\n\
                                                **CPUs**: {}x {} MHz\n\
                                                **Load**: {:.2}% {:.2}% {:.2}%\n\
                                                **Processes**: {}\n\
                                                **Memory**: {:.2} / {:.2} MB used\n\
                                                **Swap**: {:.2} / {:.2} MB used", 
                                                os_type().unwrap(), os_release().unwrap(),
                                                cpu_num().unwrap(), cpu_speed().unwrap(),
                                                load.one*100.0, load.five*100.0, load.fifteen*100.0,
                                                proc_total().unwrap(),
                                                (mem.total - mem.free) / 1024, mem.total / 1024,
                                                (mem.swap_total - mem.swap_free) / 1024, mem.swap_total / 1024,
                                            ), false),
                ])
                .footer(|f| {
                    f.text("Made with ❤️ by Moka#0002~")
                })
        })
    });

    Ok(())
}
