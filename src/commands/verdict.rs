use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::models::{
    matches::*,
    summoner::*,
};

use crate::utils::{
    riot::*,
};

use reqwest;

#[command]
fn verdict(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    // Start Message
    let _ = msg.channel_id.say(
        &ctx.http,
        "Give me a second while I look this summoner up...",
    );

    // Intialize a blocking client for now
    let client = reqwest::blocking::Client::new();

    // Build Summonner info Url and the get it
    let url: String = get_riot_url(
        "na1",
        "summoner/v4/summoners",
        "by-name",
        args.rest(),
        &mut "".into(),
    );

    let summoner_info = client
        .get(reqwest::Url::parse(&url[..]).unwrap())
        .send()?
        .json::<Summoner>()?;

    // Build Match infor url and get it
    let url: String = get_riot_url(
        "na1",
        "match/v4/matchlists",
        "by-account",
        &summoner_info.accountId[..],
        &mut "endIndex=15".into(),
    );

    let match_list = client
        .get(reqwest::Url::parse(&url[..]).unwrap())
        .send()?
        .json::<MatchList>()?;

    let _ = msg.channel_id.say(
        &ctx.http,
        format!(
            "Alright I have here their last {} matches. Let's see how they did...",
            match_list.matches.len()
        ),
    );

    // Lets get the individual match data to pull more insight
    for match_info in match_list.matches.iter() {

        // Build match url
        let url: String = get_riot_url(
            "na1",
            "match/v4",
            "matches",
            &format!("{}", match_info.game_id)[..],
            &mut "".into(),
        );

        let _ = msg.channel_id.say(
            &ctx.http,
            format!("URL: {}", url)
        );

        // Get Match Infodevweb
        match client
        .get(reqwest::Url::parse(&url[..]).unwrap())
        .send()?
        .json::<Match>() {
            Ok(_) => { msg.channel_id.say(
                &ctx.http,
                format!("URL: {}", url)
            )?;},
            Err(e) => {msg.channel_id.say(
                &ctx.http,
                format!("URL: {}", e)
            )?;
            return Ok(())
            }
        }

        // let _ = msg.channel_id.say(
        //     &ctx.http,
        //     format!(
        //         "{} - {}",
        //         current_match.gameId,
        //         current_match.gameMode
        //     ),
        // );
    }

    let _ = msg.channel_id.say(
        &ctx.http,
        "done"
    );

    Ok(())
}
