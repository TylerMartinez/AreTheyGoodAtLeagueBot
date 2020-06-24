use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
fn verdict(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.say(&ctx.http, "Still working on who to ask bruh");

    Ok(())
}