
pub fn get_riot_url() -> String {
    // Get token
    let riot_token = std::env::var("RIOT_TOKEN")
        .expect("No token found!");
    
    format!("https://{}.api.riotgames.com/lol/{}/{}/{}?api_key={}", "na1", "summoner/v4/summoners", "by-name", "robotgtar", riot_token)
}
