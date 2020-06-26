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

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct MatchList {
    pub startIndex: i32,
    pub totalGames: i32,
    pub endIndex: i32,
    pub matches: Vec<MatchReference>,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct MatchReference {
    pub gameId: i64,
    pub role: String,
    pub season: i32,
    pub platformId: String,
    pub champion: i32,
    pub queue: i32,
    pub lane: String,
    pub timestamp: i64,
}

// Functions
pub fn get_riot_url(
    region: &str,
    service: &str,
    endpoint: &str,
    resource: &str,
    query: &mut String,
) -> String {
    // Get token
    let riot_token = std::env::var("RIOT_TOKEN").expect("No token found!");

    // Add additional format if query has a value
    if query.len() > 0 {
        query.push('&');
    }

    format!(
        "https://{}.api.riotgames.com/lol/{}/{}/{}?{}api_key={}",
        region, service, endpoint, resource, query, riot_token
    )
}
