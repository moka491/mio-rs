mod commands;

use log::{error, info};
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::StandardFramework,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use std::{collections::HashSet, env, sync::Arc};

struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;
impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    kankyo::load().expect("Failed to load .env file");

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.owners(owners).prefix("~"))
            .group(&commands::hololive::HOLOLIVE_GROUP)
            .group(&commands::tldr::TLDR_GROUP)
            .after(|ctx, msg, command_name, error| match error {
                Ok(()) => println!("Processed command '{}'", command_name),
                Err(why) => {
                    println!("Command '{}' returned error {:?}", command_name, why);
                    let _ = msg.channel_id.say(
                        &ctx.http,
                        format!("Command '{}' returned error {:?}", command_name, why),
                    );
                }
            }),
    );

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}
