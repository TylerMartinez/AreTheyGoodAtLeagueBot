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

pub fn get_champion_name(id: i32) -> String {
    match id {
        266 => "Aatrox".into(),
        103 => "Ahri".into(),
        84 => "Akali".into(),
        12 => "Alistar".into(),
        32 => "Amumu".into(),
        34 => "Anivia".into(),
        1 => "Annie".into(),
        523 => "Aphelios".into(),
        22 => "Ashe".into(),
        136 => "AurelionSol".into(),
        268 => "Azir".into(),
        432 => "Bard".into(),
        53 => "Blitzcrank".into(),
        63 => "Brand".into(),
        201 => "Braum".into(),
        51 => "Caitlyn".into(),
        164 => "Camille".into(),
        69 => "Cassiopeia".into(),
        31 => "Chogath".into(),
        42 => "Corki".into(),
        122 => "Darius".into(),
        131 => "Diana".into(),
        119 => "Draven".into(),
        36 => "DrMundo".into(),
        245 => "Ekko".into(),
        60 => "Elise".into(),
        28 => "Evelynn".into(),
        81 => "Ezreal".into(),
        9 => "Fiddlesticks".into(),
        114 => "Fiora".into(),
        105 => "Fizz".into(),
        3 => "Galio".into(),
        41 => "Gangplank".into(),
        86 => "Garen".into(),
        150 => "Gnar".into(),
        79 => "Gragas".into(),
        104 => "Graves".into(),
        120 => "Hecarim".into(),
        74 => "Heimerdinger".into(),
        420 => "Illaoi".into(),
        39 => "Irelia".into(),
        427 => "Ivern".into(),
        40 => "Janna".into(),
        59 => "JarvanIV".into(),
        24 => "Jax".into(),
        126 => "Jayce".into(),
        202 => "Jhin".into(),
        222 => "Jinx".into(),
        145 => "Kaisa".into(),
        429 => "Kalista".into(),
        43 => "Karma".into(),
        30 => "Karthus".into(),
        38 => "Kassadin".into(),
        55 => "Katarina".into(),
        10 => "Kayle".into(),
        141 => "Kayn".into(),
        85 => "Kennen".into(),
        121 => "Khazix".into(),
        203 => "Kindred".into(),
        240 => "Kled".into(),
        96 => "KogMaw".into(),
        7 => "Leblanc".into(),
        64 => "LeeSin".into(),
        89 => "Leona".into(),
        127 => "Lissandra".into(),
        236 => "Lucian".into(),
        117 => "Lulu".into(),
        99 => "Lux".into(),
        54 => "Malphite".into(),
        90 => "Malzahar".into(),
        57 => "Maokai".into(),
        11 => "MasterYi".into(),
        21 => "MissFortune".into(),
        62 => "MonkeyKing".into(),
        82 => "Mordekaiser".into(),
        25 => "Morgana".into(),
        267 => "Nami".into(),
        75 => "Nasus".into(),
        111 => "Nautilus".into(),
        518 => "Neeko".into(),
        76 => "Nidalee".into(),
        56 => "Nocturne".into(),
        20 => "Nunu".into(),
        2 => "Olaf".into(),
        61 => "Orianna".into(),
        516 => "Ornn".into(),
        80 => "Pantheon".into(),
        78 => "Poppy".into(),
        555 => "Pyke".into(),
        246 => "Qiyana".into(),
        133 => "Quinn".into(),
        497 => "Rakan".into(),
        33 => "Rammus".into(),
        421 => "RekSai".into(),
        58 => "Renekton".into(),
        107 => "Rengar".into(),
        92 => "Riven".into(),
        68 => "Rumble".into(),
        13 => "Ryze".into(),
        113 => "Sejuani".into(),
        235 => "Senna".into(),
        875 => "Sett".into(),
        35 => "Shaco".into(),
        98 => "Shen".into(),
        102 => "Shyvana".into(),
        27 => "Singed".into(),
        14 => "Sion".into(),
        15 => "Sivir".into(),
        72 => "Skarner".into(),
        37 => "Sona".into(),
        16 => "Soraka".into(),
        50 => "Swain".into(),
        517 => "Sylas".into(),
        134 => "Syndra".into(),
        223 => "TahmKench".into(),
        163 => "Taliyah".into(),
        91 => "Talon".into(),
        44 => "Taric".into(),
        17 => "Teemo".into(),
        412 => "Thresh".into(),
        18 => "Tristana".into(),
        48 => "Trundle".into(),
        23 => "Tryndamere".into(),
        4 => "TwistedFate".into(),
        29 => "Twitch".into(),
        77 => "Udyr".into(),
        6 => "Urgot".into(),
        110 => "Varus".into(),
        67 => "Vayne".into(),
        45 => "Veigar".into(),
        161 => "Velkoz".into(),
        254 => "Vi".into(),
        112 => "Viktor".into(),
        8 => "Vladimir".into(),
        106 => "Volibear".into(),
        19 => "Warwick".into(),
        498 => "Xayah".into(),
        101 => "Xerath".into(),
        5 => "XinZhao".into(),
        157 => "Yasuo".into(),
        83 => "Yorick".into(),
        350 => "Yuumi".into(),
        154 => "Zac".into(),
        238 => "Zed".into(),
        115 => "Ziggs".into(),
        26 => "Zilean".into(),
        142 => "Zoe".into(),
        143 => "Zyra".into(),
        _ => "a champion I have never heard of".into(),
    }
}