use serde::Deserialize;
use std::collections::HashMap;

// Models
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchList {
    #[serde(default)]
    pub start_index: i32,

    #[serde(default)]
    pub total_games: i32,

    #[serde(default)]
    pub end_index: i32,

    #[serde(default)]
    pub matches: Vec<MatchReference>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchReference {
    #[serde(default)]
    pub game_id: i64,

    #[serde(default)]
    pub role: String,

    #[serde(default)]
    pub season: i32,

    #[serde(default)]
    pub platform_id: String,

    #[serde(default)]
    pub champion: i32,

    #[serde(default)]
    pub queue: i32,

    #[serde(default)]
    pub lane: String,

    #[serde(default)]
    pub timestamp: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    #[serde(default)]
    pub game_id: i64,

    #[serde(default)]
    pub participant_identities: Vec<ParticipantIdentity>,

    #[serde(default)]
    pub queue_id: i32,

    #[serde(default)]
    pub game_type: String,

    #[serde(default)]
    pub game_duration: i64,

    #[serde(default)]
    pub teams: Vec<TeamStats>,

    #[serde(default)]
    pub platform_id: String,

    #[serde(default)]
    pub game_creation: i64,

    #[serde(default)]
    pub season_id: i32,

    #[serde(default)]
    pub game_version: String,

    #[serde(default)]
    pub map_id: i32,

    #[serde(default)]
    pub game_mode: String,

    #[serde(default)]
    pub participants: Vec<Participant>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantIdentity {
    #[serde(default)]
    pub participant_id: i32,

    pub player: Player,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    #[serde(default)]
    pub profile_icon: i32,

    #[serde(default)]
    pub account_id: String,

    #[serde(default)]
    pub match_history_uri: String,

    #[serde(default)]
    pub current_account_id: String,

    #[serde(default)]
    pub current_platform_id: String,

    #[serde(default)]
    pub summoner_name: String,

    #[serde(default)]
    pub summoner_id: String,

    #[serde(default)]
    pub platform_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamStats {
    #[serde(default)]
    pub tower_kills: i32,

    #[serde(default)]
    pub rift_herald_kills: i32,

    #[serde(default)]
    pub first_blood: bool,

    #[serde(default)]
    pub inhibitor_kills: i32,

    #[serde(default)]
    pub bans: Vec<TeamBans>,

    #[serde(default)]
    pub first_baron: bool,

    #[serde(default)]
    pub first_dragon: bool,

    #[serde(default)]
    pub dominion_victory_score: i32,

    #[serde(default)]
    pub dragon_kills: i32,

    #[serde(default)]
    pub baron_kills: i32,

    #[serde(default)]
    pub first_inhibitor: bool,

    #[serde(default)]
    pub first_tower: bool,

    #[serde(default)]
    pub vilemaw_kills: i32,

    #[serde(default)]
    pub first_rift_herald: bool,

    #[serde(default)]
    pub team_id: i32,

    #[serde(default)]
    pub win: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamBans {
    #[serde(default)]
    pub champion_id: i32,

    #[serde(default)]
    pub pick_turn: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    #[serde(default)]
    pub participant_id: i32,

    #[serde(default)]
    pub champion_id: i32,

    #[serde(default)]
    pub runes: Vec<Rune>,

    pub stats: ParticipantStats,

    #[serde(default)]
    pub team_id: i32,

    pub timeline: ParticipantTimeline,

    #[serde(default)]
    pub spell1_id: i32,

    #[serde(default)]
    pub spell2_id: i32,

    #[serde(default)]
    pub highest_achieved_season_tier: String,

    #[serde(default)]
    pub masteries: Vec<Mastery>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rune {
    #[serde(default)]
    pub rune_id: i32,

    #[serde(default)]
    pub rank: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantStats {
    #[serde(default)]
    pub item0: i32,

    #[serde(default)]
    pub item2: i32,
    #[serde(default)]
    pub total_units_healed: i32,
    #[serde(default)]
    pub item1: i32,
    #[serde(default)]
    pub largest_multi_kill: i32,
    #[serde(default)]
    pub gold_earned: i32,

    #[serde(default)]
    pub first_inhibitor_kill: bool,

    #[serde(default)]
    pub physical_damage_taken: i64,

    #[serde(default)]
    pub node_neutralize_assist: i32,

    #[serde(default)]
    pub total_player_score: i32,

    #[serde(default)]
    pub rune_id: i32,

    #[serde(default)]
    pub champ_level: i32,

    #[serde(default)]
    pub damage_dealt_to_objectives: i64,

    #[serde(default)]
    pub total_damage_taken: i64,

    #[serde(default)]
    pub neutral_minions_killed: i32,

    #[serde(default)]
    pub deaths: i32,

    #[serde(default)]
    pub triple_kills: i32,
    #[serde(default)]
    pub magic_damage_dealt_to_champions: i64,

    #[serde(default)]
    pub wards_killed: i32,

    #[serde(default)]
    pub penta_kills: i32,
    #[serde(default)]
    pub damage_self_mitigated: i64,

    #[serde(default)]
    pub largest_critical_strike: i32,

    #[serde(default)]
    pub nodeneutralize: i32,

    #[serde(default)]
    pub total_time_crowd_control_dealt: i32,

    #[serde(default)]
    pub first_tower_kill: bool,

    #[serde(default)]
    pub magic_damage_dealt: i64,

    #[serde(default)]
    pub total_score_rank: i32,

    #[serde(default)]
    pub node_capture: i32,

    #[serde(default)]
    pub wards_placed: i32,

    #[serde(default)]
    pub total_damage_dealt: i64,

    #[serde(default)]
    pub time_c_cing_others: i64,

    #[serde(default)]
    pub magical_damage_taken: i64,

    #[serde(default)]
    pub largest_killing_spree: i32,

    #[serde(default)]
    pub total_damage_dealt_to_champions: i64,

    #[serde(default)]
    pub physical_damage_dealt_to_champions: i64,

    #[serde(default)]
    pub neutral_minions_killed_team_jungle: i32,
    #[serde(default)]
    pub first_inhibitor_assist: bool,

    #[serde(default)]
    pub vision_wards_bought_in_game: i32,

    #[serde(default)]
    pub objective_player_score: i32,

    #[serde(default)]
    pub kills: i32,

    #[serde(default)]
    pub first_tower_assist: bool,

    #[serde(default)]
    pub combat_player_score: i32,

    #[serde(default)]
    pub inhibitor_kills: i32,

    #[serde(default)]
    pub turret_kills: i32,

    #[serde(default)]
    pub participant_id: i32,

    #[serde(default)]
    pub true_damage_taken: i64,

    #[serde(default)]
    pub first_blood_assist: bool,

    #[serde(default)]
    pub node_capture_assist: i32,

    #[serde(default)]
    pub assists: i32,

    #[serde(default)]
    pub team_objective: i32,

    #[serde(default)]
    pub altars_neutralized: i32,

    #[serde(default)]
    pub gold_spent: i32,

    #[serde(default)]
    pub damage_dealt_to_turrets: i64,

    #[serde(default)]
    pub altars_captured: i32,

    #[serde(default)]
    pub win: bool,

    #[serde(default)]
    pub total_heal: i64,

    #[serde(default)]
    pub unreal_kills: i32,

    #[serde(default)]
    pub vision_score: i64,

    #[serde(default)]
    pub physical_damage_dealt: i64,

    #[serde(default)]
    pub first_blood_kill: bool,

    #[serde(default)]
    pub longest_time_spent_living: i32,

    #[serde(default)]
    pub killing_sprees: i32,

    #[serde(default)]
    pub sight_wards_bought_in_game: i32,

    #[serde(default)]
    pub true_damage_dealt_to_champions: i64,

    #[serde(default)]
    pub neutral_minions_killed_enemy_jungle: i32,

    #[serde(default)]
    pub double_kills: i32,

    #[serde(default)]
    pub true_damage_dealt: i64,

    #[serde(default)]
    pub quadra_kills: i32,

    #[serde(default)]
    pub item4: i32,

    #[serde(default)]
    pub item3: i32,

    #[serde(default)]
    pub item6: i32,

    #[serde(default)]
    pub item5: i32,

    #[serde(default)]
    pub player_score0: i32,

    #[serde(default)]
    pub player_score1: i32,

    #[serde(default)]
    pub player_score2: i32,

    #[serde(default)]
    pub player_score3: i32,

    #[serde(default)]
    pub player_score4: i32,

    #[serde(default)]
    pub player_score5: i32,

    #[serde(default)]
    pub player_score6: i32,

    #[serde(default)]
    pub player_score7: i32,

    #[serde(default)]
    pub player_score8: i32,

    #[serde(default)]
    pub player_score9: i32,

    #[serde(default)]
    pub perk0: i32,

    #[serde(default)]
    pub perk0_var1: i32,

    #[serde(default)]
    pub perk0_var2: i32,

    #[serde(default)]
    pub perk0_var3: i32,

    #[serde(default)]
    pub perk1: i32,

    #[serde(default)]
    pub perk1_var1: i32,

    #[serde(default)]
    pub perk1_var2: i32,

    #[serde(default)]
    pub perk1_var3: i32,

    #[serde(default)]
    pub perk2: i32,

    #[serde(default)]
    pub perk2_var1: i32,

    #[serde(default)]
    pub perk2_var2: i32,

    #[serde(default)]
    pub perk2_var3: i32,

    #[serde(default)]
    pub perk3: i32,

    #[serde(default)]
    pub perk3_var1: i32,

    #[serde(default)]
    pub perk3_var2: i32,
    #[serde(default)]
    pub perk3_var3: i32,

    #[serde(default)]
    pub perk4: i32,

    #[serde(default)]
    pub perk4_var1: i32,

    #[serde(default)]
    pub perk4_var2: i32,

    #[serde(default)]
    pub perk4_var3: i32,

    #[serde(default)]
    pub perk5: i32,

    #[serde(default)]
    pub perk5_var1: i32,

    #[serde(default)]
    pub perk5_var2: i32,

    #[serde(default)]
    pub perk5_var3: i32,

    #[serde(default)]
    pub perk_primary_style: i32,

    #[serde(default)]
    pub perk_sub_style: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantTimeline {
    #[serde(default)]
    pub participant_id: i32,

    #[serde(default)]
    pub cs_diff_per_min_deltas: HashMap<String, f64>,

    #[serde(default)]
    pub damage_taken_per_min_deltas: HashMap<String, f64>,

    #[serde(default)]
    pub role: String,

    #[serde(default)]
    pub damage_taken_diff_per_min_deltas: HashMap<String, f64>,

    #[serde(default)]
    pub xp_per_min_deltas: HashMap<String, f64>,

    #[serde(default)]
    pub xp_diff_per_min_deltas: HashMap<String, f64>,

    #[serde(default)]
    pub lane: String,

    #[serde(default)]
    pub creeps_per_min_deltas: HashMap<String, f64>,

    #[serde(default)]
    pub gold_per_min_deltas: HashMap<String, f64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mastery {
    #[serde(default)]
    pub rank: i32,

    #[serde(default)]
    pub mastery_id: i32,
}

// Functions
