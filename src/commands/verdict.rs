use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use reqwest;

#[command]
fn verdict(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.say(&ctx.http, "Trying to post...");

    let client = reqwest::Client::new();

    

    Ok(())
}