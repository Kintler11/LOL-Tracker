// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::{Deserialize, Serialize};
mod lol_api;
use lol_api::model::{*};
use serde_json::json;
use std::{fs::File, io::Read, env, sync::Mutex};
use lazy_static::lazy_static;
lazy_static! {
    static ref api: Mutex<API> = Mutex::new(API::new());
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize )]
struct PlayerData{
    player_name: String,
    team: Team,
    lane: Lane,
    is_bot: bool,
    summoner_one: String,
    summoner_two: String,
    champion: Champion,
    score: PlayerScores,
    stats: PlayerPoints
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize )]
struct PlayerScores{
    level: i16,
    kills: i16,
    deaths: i16,
    assists: i16,
    creep_score: i16,
    vision_score: f32
}
#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize )]
struct PlayerPoints{
    structures: u8,
    objectives: u8,
    pentas: u8,
    first_blood: u8,
    points: u8
}

#[tauri::command]
fn update(){
    api.lock().unwrap().update();
}

#[tauri::command]
fn set_snapshot(snapshot_type: &str){
    let mut _api = api.lock().unwrap();

    _api.set_snapshot(Gametype::from_str(snapshot_type));
    _api.update();
}

#[tauri::command]
fn clear_snapshot() {
    println!("Clearing data");
    let mut _api = api.lock().unwrap();
    _api.debug_mode = false;
    _api.debug_data = None;
    _api.players = Vec::new();
    _api.events = Vec::new();
}

#[tauri::command]
fn get_all_events() -> String {
    let events = &api.lock().unwrap().events.iter().map(|event|{
        match event {
            Events::Turret { team, lane }         => json!({"type": "Turret", "team": team, "lane": lane}),
            Events::Inhibitor { team, lane }      => json!({"type": "Inhibitor", "team": team, "lane": lane}),
            Events::NexusTurret { team }                 => json!({"type": "NexusTurret", "team": team}),
            Events::Elder { team, stolen }        => json!({"type": "Elder", "team": team, "stolen": stolen}),
            Events::Dragon { team, stolen }       => json!({"type": "Dragon", "team": team, "stolen": stolen}),
            Events::Baron { team, stolen }        => json!({"type": "Baron", "team": team, "stolen": stolen}),
            Events::Herald { team, stolen }       => json!({"type": "Herald", "team": team, "stolen": stolen}),
            Events::FirstBlood { team, player } => json!({"type": "FirstBlood", "team": team, "player": player.name}),
            Events::Penta { team, player }      => json!({"type": "Penta", "team": team, "player": player.name})
        }
    }).collect::<Vec<serde_json::Value>>();
    serde_json::to_string(&events).unwrap()
}

fn get_events(player_: &Player) -> PlayerPoints{
    let api_ref = api.lock().unwrap();
    let mut player_turrets:u8 = 0;
    let mut player_objectives:u8 = 0;
    let mut player_pentas:u8 = 0;
    let mut player_deaths:u8 = 0;
    let mut player_points_count = 0;
    for event in &api_ref.events{
        match event {
            Events::Dragon { team, stolen } | Events::Baron { team, stolen } => {
                if team != &player_.team && (player_.lane == Lane::Jungle || stolen.to_owned()) {
                    player_objectives+=1; 
                    player_points_count+=1;
                }
            },
            Events::Turret { team, lane } | Events::Inhibitor { team, lane } => {
                if team == &player_.team && (lane == &player_.lane || (lane == &Lane::Bottom && player_.lane == Lane::Support)) {
                    player_turrets+=1; 
                    player_points_count+=1;
                }
            },
            Events::Herald { team, stolen } => {
                if team != &player_.team && player_.lane == Lane::Jungle {
                    player_objectives+=1; 
                    player_points_count+=1;
                }
            },
            Events::NexusTurret { team } => {
                if team == &player_.team {
                    player_turrets+=1; 
                    player_points_count+=2;
                }
            },
            Events::Elder { team, stolen } => {
                if team != &player_.team {
                    player_objectives+=1; 
                    player_points_count+=2;
                }
            },
            Events::Penta { team, player } => {
                if team != &player_.team {
                    player_pentas+=1; 
                    player_points_count+=4;
                }
            },
            Events::FirstBlood { team, player } => {
                if player.name == player_.name {
                    player_deaths+=1; 
                    player_points_count+=1;
                }
            }
        };
    }

    PlayerPoints { 
        structures: player_turrets, 
        objectives: player_objectives, 
        pentas: player_pentas, 
        first_blood: player_deaths, 
        points: player_points_count 
    }
}

#[tauri::command]
fn get_champions() -> String {
    let champions = api.lock().unwrap().champions.to_vec();
    serde_json::to_string(&champions).unwrap()
}

#[tauri::command]
fn get_random_champion(lane_index: usize) -> String {
    let lane = match lane_index {
        0 => Lane::None,
        1 => Lane::Top,
        2 => Lane::Jungle,
        3 => Lane::Middle,
        4 => Lane::Bottom,
        _ => Lane::Support
    };
    let champion = api.lock().unwrap().get_random_champion_by_lane(&lane);
    serde_json::to_string(&champion).unwrap()
}

#[tauri::command]
fn get_all_players() -> String {
    
    // Locking the thread for player data processing freezes the program indefinitely, so we make a copy instead ¯\_(ツ)_/¯
    let players = api.lock().unwrap().players.to_vec();

    let all_player_data = players.iter()
    .map(|player_data| {
        PlayerData { 
            player_name: player_data.name.clone(),
            team: player_data.team.clone(),
            lane: player_data.lane.clone(),
            is_bot: player_data.ancestor.isBot,
            summoner_one: player_data.ancestor.summonerSpells.summonerSpellOne.clone().unwrap_or(ClientApiPlayerSummonerSpell::default()).displayName,  
            summoner_two: player_data.ancestor.summonerSpells.summonerSpellTwo.clone().unwrap_or(ClientApiPlayerSummonerSpell::default()).displayName,
            champion: player_data.champion.clone(),
            score: PlayerScores {
                level: player_data.ancestor.level,
                kills: player_data.ancestor.scores.kills, 
                deaths: player_data.ancestor.scores.deaths, 
                assists: player_data.ancestor.scores.assists, 
                creep_score: player_data.ancestor.scores.creepScore,
                vision_score: player_data.ancestor.scores.wardScore 
            }, 
            stats: get_events(&player_data) 
        }
    }).collect::<Vec<PlayerData>>();

    serde_json::to_string(&all_player_data).unwrap()
}


fn main(){

    // Start the lol API
    api.lock().unwrap().start();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_random_champion, get_champions, set_snapshot, clear_snapshot, get_all_events, update, get_all_players])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
