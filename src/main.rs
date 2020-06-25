// Mods
mod commands;

// Imports
use std::{
    collections::HashSet,
    env,
    sync::Arc,
};

use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{
        StandardFramework,
        standard::macros::group,
    },
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};

use log::{info};

use commands::{
    verdict::*,
};

// Structs
struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

// Command Groups
#[group]
#[commands(verdict)]
struct General;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up ENVs
    kankyo::load().expect("Couldn't load .env file");

    // Initiate logger
    env_logger::init();

    // Get the bot token
    let token = env::var("DISCORD_TOKEN")
        .expect("No token found!");
    
    // Get the client
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    // Set up shard manager
    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    // Get owners
    let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        },
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };
    
    // Set up standard framework
    client.with_framework(StandardFramework::new()
        .configure(|c| c
            .owners(owners)
            .prefix("!"))
        .group(&GENERAL_GROUP));

    // Start the client
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }

    Ok(())
}
