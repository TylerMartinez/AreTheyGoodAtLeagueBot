use serde::Deserialize;

// Models
#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct Summoner {
    pub accountId: String,
    pub profileIconId: i32,
    pub revisionDate: i64,
    pub name: String,
    pub id: String,
    pub puuid: String,
    pub summonerLevel: i64,
}