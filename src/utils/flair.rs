use rand::seq::SliceRandom;

// Functions
pub fn get_nickname() -> String {
    // Nicknames
    let nicknames = vec![
        "bud", "champ", "sport", "my guy", "bruh", "dude", "kid", "son",
    ];

    format!("{}", nicknames.choose(&mut rand::thread_rng()).unwrap())
}
