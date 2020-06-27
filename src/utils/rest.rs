// Imports
use crate::utils::flair::*;
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use serenity::model::prelude::*;
use serenity::prelude::*;

// Functions
pub fn make_get_request<T: DeserializeOwned>(
    client: &Client,
    url: String,
    ctx: &mut Context,
    msg: &Message,
    error_text: String,
) -> Result<T, ()> {
    let response = client
        .get(reqwest::Url::parse(&url[..]).unwrap())
        .send()
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => return Ok(response.json::<T>().unwrap()),
        _ => {
            let _ = msg
                .channel_id
                .say(&ctx.http, format!("{} {}", error_text, get_nickname()));
            return Err(());
        }
    };
}
