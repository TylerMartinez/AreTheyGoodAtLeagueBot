use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg:Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") {
                println!("Errored sending message: {}", why)
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("No token found!");
    
        let mut client = Client::new(&token, Handler).expect("Issue creating client!");


        if let Err(why) = client.start() {
            println!("Client error: {:?}", why);
        }
}
