use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

use crate::commands::riot;

use reqwest;


#[command]
fn verdict(ctx: &mut Context, msg: &Message) -> CommandResult {
    // Start Message
    let _ = msg.channel_id.say(&ctx.http, "Trying to GET...");

    // Build Url 
    let url:String = riot::get_riot_url();

    let _ = msg.channel_id.say(&ctx.http, format!("..to this url: {}...", url));

    let client = reqwest::blocking::Client::new();

    let response = client.get(reqwest::Url::parse(&url[..]).unwrap())
        .send()?
        .text()?;

    let _ = msg.channel_id.say(&ctx.http, "..and got:");

    let _ = msg.channel_id.say(&ctx.http, response);

    Ok(())
}