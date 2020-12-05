extern crate rustc_version_runtime;
use rustc_version_runtime::version;
use sysinfo::{ProcessExt, ProcessorExt, SystemExt};

use crate::core::context::{StartTimeContainer, SysInfoContainer};
use chrono::Utc;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};
const BOT_VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[command]
pub async fn info(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let app_info = &ctx.http.get_current_application_info().await?;
    let bot_id = &ctx.http.get_current_user().await?;

    let bot_user = &ctx.cache.user(bot_id).await.unwrap();
    let bot_avatar = bot_user.avatar_url().unwrap_or(String::from(""));

    // Refresh system info
    {
        let mut data = ctx.data.write().await;
        let sys = data.get_mut::<SysInfoContainer>().unwrap();
        sys.refresh_all();
    }

    // Uptime calculation
    let data = ctx.data.read().await;
    let start_time = data.get::<StartTimeContainer>().unwrap();
    let bot_uptime = Utc::now().signed_duration_since(*start_time).num_seconds();

    // System info
    let sys = data.get::<SysInfoContainer>().unwrap();
    let cpu = sys.get_global_processor_info();
    let cpu_name = cpu.get_name().trim_end_matches("Total CPU").trim_end();
    let cpu_count = sys.get_processors().len();
    let avg_cpu_frequency = sys
        .get_processors()
        .iter()
        .fold(0, |freq, p| freq + p.get_frequency() / cpu_count as u64);

    let bot_pid = sysinfo::get_current_pid().unwrap();
    let bot_process = sys.get_process(bot_pid).unwrap();

    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("About Ookami Mio")
                .description(
                    "**こんばんみぉーん**！\n\
                    I'm Ookami Mio, your fellow helper bot written in [Rust](https://www.rust-lang.org/) using [serenity](https://github.com/serenity-rs/serenity).",
                )
                .thumbnail(bot_avatar)
                .fields(vec![
                    ("Bot Info", 
                        format!("
                            **Version**: v{}\n\
                            **Compiled with**: rustc v{}\n\
                            **Owner**: {}#{:04}\n\
                            **Mem usage**: {:.2} MB\n\
                            **Uptime**: {}",
                            BOT_VERSION,
                            version(),
                            app_info.owner.name, app_info.owner.discriminator,
                            bot_process.memory() / 1024,
                            get_formatted_uptime(bot_uptime as u64),

                        ), true),

                    ("Dependencies", 
                        "**tokio**: v0.2\n\
                        **serenity-rs**: v0.9.2\n\
                        **reqwest**: v0.10"
                        .to_string(), true),

                    ("System Info", 
                        format!(
                            "**CPU**: {} {}, {}x {:.2} MHz\n\
                            **CPU usage**: {:.2}%\n\
                            **RAM usage**: {} / {} MB\n\
                            **Swap usage**: {} / {} MB\n\
                            **System uptime**: {}",
                            cpu.get_brand(), cpu_name, cpu_count, avg_cpu_frequency,
                            cpu.get_cpu_usage(),
                            sys.get_used_memory() / 1024, sys.get_total_memory() / 1024,
                            sys.get_used_swap() / 1024, sys.get_total_swap() / 1024,
                            get_formatted_uptime(sys.get_uptime())
                        ), false),
                ])
                .footer(|f| {
                    f.text("Made with ❤️ by Moka#0002~")
                })
        })
    }).await;

    Ok(())
}

fn get_formatted_uptime(total_seconds: u64) -> String {
    let week_r = total_seconds % 604800;
    let day_r = week_r % 86400;
    let hour_r = day_r % 3600;
    let minute_r = hour_r % 60;

    let up_weeks = (total_seconds - week_r) / 604800;
    let up_days = (week_r - day_r) / 86400;
    let up_hours = (day_r - hour_r) / 3600;
    let up_minutes = (hour_r - minute_r) / 60;
    let up_seconds = minute_r % 60;

    format!(
        "{}w {}d {}h {}m {}s",
        up_weeks, up_days, up_hours, up_minutes, up_seconds
    )
}
