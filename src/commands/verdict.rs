use std::collections::HashMap;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use chrono::prelude::*;
use chrono::Utc;

use crate::models::{matches::*, summoner::*};

use crate::utils::{rest::*, riot::*};

use reqwest;

#[command]
fn verdict(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    // Start Message
    let _ = msg.channel_id.say(
        &ctx.http,
        "Give me a second while I look this summoner up...",
    );

    let name = args.single::<String>()?;
    let option = match args.single::<String>() {
        Ok(x) => x,
        Err(_) => "".into(),
    };

    // Intialize a blocking client for now
    let client = reqwest::blocking::Client::new();

    // Build Summonner info Url and the get it
    let url: String = get_riot_url(
        "na1",
        "summoner/v4/summoners",
        "by-name",
        &name[..],
        &mut "".into(),
    );

    let summoner_info = match make_get_request::<Summoner>(
        &client,
        url,
        ctx,
        msg,
        "... couldn't find this summoner's info".into(),
    ) {
        Ok(info) => info,
        Err(_) => return Ok(()),
    };

    // Build Match infor url and get it
    let url: String = get_riot_url(
        "na1",
        "match/v4/matchlists",
        "by-account",
        &summoner_info.accountId[..],
        &mut "queue=700&queue=440&queue=430&queue=420&queue=400&endIndex=15".into(),
    );

    let match_list = match make_get_request::<MatchList>(
        &client,
        url,
        ctx,
        msg,
        "... couldn't find this summoner's match info".into(),
    ) {
        Ok(info) => info,
        Err(_) => return Ok(()),
    };

    let _ = msg.channel_id.say(
        &ctx.http,
        format!(
            "Alright I have here their last {} matches. Let's see how they did...",
            match_list.matches.len()
        ),
    );

    // Get champ information http://ddragon.leagueoflegends.com/cdn/10.13.1/data/en_US/champion.json

    // Datapoints
    let mut wins = HashMap::<String, i32>::new();
    let mut roles = HashMap::<String, i32>::new();
    let mut champs = HashMap::<i32, i32>::new();

    // If they ask for details lets tell them
    if option == "detailed" {
        let _ = msg.channel_id.say(
            &ctx.http,
            "Here are the exact match details becuase you don't believe me:",
        );
    }

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

        // Get Match info
        let current_match = match make_get_request::<Match>(
            &client,
            url,
            ctx,
            msg,
            "... had trouble getting this summoner's match details".into(),
        ) {
            Ok(info) => info,
            Err(_) => return Ok(()),
        };

        // Find users participant id
        let participant_identity = current_match
            .participant_identities
            .iter()
            .find(|x| x.player.account_id == summoner_info.accountId)
            .unwrap();

        let participant = current_match
            .participants
            .iter()
            .find(|x| x.participant_id == participant_identity.participant_id)
            .unwrap();

        let team_stats = current_match
            .teams
            .iter()
            .find(|x| x.team_id == participant.team_id)
            .unwrap();

        // Record data
        *champs.entry(match_info.champion).or_insert(0) += 1;
        *roles
            .entry(format!("{} {}", match_info.role, match_info.lane))
            .or_insert(0) += 1;
        *wins.entry(team_stats.win[..].into()).or_insert(0) += 1;

        // Print details if wanted
        if option == "detailed" {
            let naive = NaiveDateTime::from_timestamp(match_info.timestamp / 1000, 0);
            let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
            let newdate = datetime.format("%Y-%m-%d %H:%M:%S");

            let _ = msg.channel_id.say(
                &ctx.http,
                format!(
                    "{} - {} - {} - {} - {}",
                    newdate,
                    get_queue_name(match_info.queue),
                    format!("{} {}", match_info.role, match_info.lane),
                    get_champion_name(match_info.champion),
                    team_stats.win
                ),
            );
        }
    }

    // Print some stats
    let _ = msg
        .channel_id
        .say(&ctx.http, "...alright some quick stats:");

    let _ = msg.channel_id.say(
        &ctx.http,
        format!(
            "Their most played champ is {}",
            get_champion_name(
                champs
                    .into_iter()
                    .max_by_key(|&(_, count)| count)
                    .map(|(val, _)| val)
                    .unwrap()
            )
        ),
    );

    let _ = msg.channel_id.say(
        &ctx.http,
        format!(
            "Favorite lane/role combo is {}",
            roles
                .into_iter()
                .max_by_key(|&(_, count)| count)
                .map(|(val, _)| val)
                .unwrap()
        ),
    );

    // Render verdict
    let _ = msg.channel_id.say(&ctx.http, "Alright so my verdict is...");

    let win_count = *wins.entry("Win".into()).or_insert(0) as f32;

    let win_rate = win_count / 15.0;

    let percent = format!("{:.2}", win_rate)[2..].to_string();

    if win_rate > 0.9 {
        let _ = msg.channel_id.say(
            &ctx.http,
            format!(
                "{} is insanely good at League with a winrate of {}%",
                name, percent
            ),
        );
    } else if win_rate > 0.75 {
        let _ = msg.channel_id.say(
            &ctx.http,
            format!(
                "{} is pretty good at League with a winrate of {}%",
                name, percent
            ),
        );
    } else if win_rate > 0.5 {
        let _ = msg.channel_id.say(
            &ctx.http,
            format!(
                "{} is decent at League with a winrate of {}%",
                name, percent
            ),
        );
    } else if win_rate > 0.25 {
        let _ = msg.channel_id.say(
            &ctx.http,
            format!(
                "{} is not that good at League with a winrate of {}%",
                name, percent
            ),
        );
    } else if win_rate > 0.1 {
        let _ = msg.channel_id.say(
            &ctx.http,
            format!("{} is bad at League with a winrate of {}%", name, percent),
        );
    } else {
        let _ = msg.channel_id.say(
            &ctx.http,
            format!("{} is trash at League with a winrate of {}%", name, percent),
        );
    }

    Ok(())
}
