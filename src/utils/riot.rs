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
