use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};
use reqwest;


#[derive(PartialEq, Eq, Debug, Serialize )]
pub enum Gametype{
    Normal,
    ARAM,
    Arena,
    TFT,
    UltimateSpellBook,
    None
}

impl Gametype {
    pub fn from_str(gametype: &str) -> Gametype{
        match gametype.to_lowercase().as_str(){
            "aram" => Gametype::ARAM,
            "arena" => Gametype::Arena,
            "tft" => Gametype::TFT,
            "spellbook" => Gametype::UltimateSpellBook,
            _ => Gametype::Normal
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize )]
pub enum Kill{
    FirstBlood,
    Double,
    Triple,
    Quadra,
    Penta
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Serialize )]
pub enum Events {
    NexusTurret{team: Team},
    Turret{team: Team, lane: Lane},
    Inhibitor{team: Team, lane: Lane},
    Dragon{team: Team, stolen: bool},
    Elder{team: Team, stolen: bool},
    Herald{team: Team, stolen: bool},
    Baron{team: Team, stolen: bool},
    Penta{team: Team, player: Player},
    FirstBlood{team: Team, player: Player}
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy, Serialize )]
pub enum Team{
    Blue,
    Red
}

impl Team{
    pub fn flip(self) -> Team{
        match self {
            Team::Blue => Team::Red,
            Team::Red => Team::Blue
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Serialize )]
pub enum Lane{
    Top,
    Jungle,
    Middle,
    Bottom,
    Support,
    None
}

// Event types


#[derive(Debug)]
pub struct API{
    pub api_client: reqwest::Client,
    pub version: String,
    pub debug_mode: bool,
    pub debug_data: Option<ClientApiGameData>,
    pub in_game: bool,
    pub game_type: Gametype,
    pub active_player: ClientApiActivePlayer,
    pub events: Vec<Events>,
    pub players: Vec<Player>,
    pub champions: Vec<Champion>,
    pub settings: API_SETTINGS
}

#[derive(Debug, Clone, Deserialize, Serialize )]
pub struct API_ENDPOINTS{
    pub game_status: String,
    pub game_data: String,
    pub game_version: String,
    pub champions: String,
    pub play_rate: String,
}

impl Default for API_ENDPOINTS{
    fn default() -> Self {
        Self { 
            game_status: "https://127.0.0.1:2999/liveclientdata/gamestats".to_string(), 
            game_data: "https://127.0.0.1:2999/liveclientdata/allgamedata".to_string(), 
            game_version: "https://ddragon.leagueoflegends.com/api/versions.json".to_string(), 
            champions: "https://ddragon.leagueoflegends.com/cdn/{VERSION}/data/en_US/champion.json".to_string(), 
            play_rate: "http://cdn.merakianalytics.com/riot/lol/resources/latest/en-US/championrates.json".to_string() 
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize )]
pub struct API_SETTINGS{
    pub endpoints: API_ENDPOINTS
}

// Parsed Fields
#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize )]
pub struct Player{
    pub name: String,
    pub champion: Champion,
    pub team: Team,
    pub lane: Lane,
    pub ancestor: ClientApiPlayer
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize )]
pub struct Champion{
    pub name: String,
    pub id: String,
    pub lanes: Vec<Lane>,
    pub image: String
}

impl Champion {
    pub fn Default(game_mode: &Gametype) -> Self{
        match game_mode {
            Gametype::TFT => Self { 
                name: "Little Legend".to_string(), 
                id: "TFT".to_string(),
                image: "tft-tactician/Tooltip_TFT_Avatar_Blue.png".to_string() ,
                lanes: vec![Lane::Middle, Lane::Top]
            },
            _ => Self { 
                name: "Aurelion Sol".to_string(), 
                id: "AurelionSol".to_string(),
                image: "champion/AurelionSol.png".to_string(),
                lanes: vec![Lane::Middle, Lane::Top]
            }
        }
        
    }
}

#[derive(Debug, Clone, Serialize )]
pub struct Multikill{
    pub kill: Kill,
    pub team: Team,
    pub player: String
}

// Api Fields

// Player data fields
#[derive(Deserialize, Debug, Clone, Serialize )]
pub struct ClientApiGameData {
    pub activePlayer: ClientApiActivePlayer,
    pub allPlayers: Vec<ClientApiPlayer>,
    pub events: ClientApiEventHeader,
    pub gameData: ClientApiGameTypeData
}

#[derive(Deserialize, Debug, Clone, Serialize )]
pub struct ClientApiGameTypeData {
    pub gameMode: String,
    pub gameTime: f32,
    pub mapName: String,
    pub mapNumber: i8,
    pub mapTerrain: String
}



#[derive(Deserialize, Debug, Default, Clone, Serialize )]
pub struct ClientApiActivePlayer{
    pub abilities: ClientApiActivePlayerAbilities,
    pub currentGold: f32,
    pub level: i16,
    pub summonerName: String
}

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Serialize )]
pub struct ClientApiPlayer{
    pub championName: String,
    pub isBot: bool,
    pub isDead: bool,
    pub items: Vec<ClientApiPlayerItem>,
    pub level: i16,
    pub position: String,
    pub rawChampionName: String,
    pub respawnTimer: f32,
    pub scores: ClientApiPlayerScores,
    pub skinID: i32,
    pub summonerName: String,
    pub summonerSpells: ClientApiPlayerSummonerSpells,
    pub team: String
}

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Serialize )]
pub struct ClientApiPlayerItem{
    pub canUse: bool,
    pub consumable: bool,
    pub count: i8,
    pub displayName: String,
    pub itemID: i32,
    pub price: i32,
    pub rawDescription: String,
    pub rawDisplayName: String,
    pub slot: i8
}

#[derive(Deserialize, Debug, Clone , PartialEq, PartialOrd, Serialize )]
pub struct ClientApiPlayerScores{
    pub assists: i16,
    pub creepScore: i16,
    pub deaths: i16,
    pub kills: i16,
    pub wardScore: f32
}

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Serialize )]
pub struct ClientApiPlayerSummonerSpells{
    pub summonerSpellOne: Option<ClientApiPlayerSummonerSpell>,
    pub summonerSpellTwo: Option<ClientApiPlayerSummonerSpell>
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, PartialOrd, Serialize )]
pub struct ClientApiPlayerSummonerSpell{
    pub displayName: String,
    pub rawDescription: String,
    pub rawDisplayName: String
}

#[derive(Deserialize, Debug, Default, Clone, Serialize )]
pub struct ClientApiActivePlayerAbilities{
    pub E: ClientApiActivePlayerAbility,
    pub Passive: ClientApiActivePlayerAbility,
    pub Q: ClientApiActivePlayerAbility,
    pub R: ClientApiActivePlayerAbility,
    pub W: ClientApiActivePlayerAbility
}

#[derive(Deserialize, Debug, Default, Clone, Serialize )]
pub struct ClientApiActivePlayerAbility{
    pub abilityLevel: Option<i8>,
    pub displayName: String,
    pub id: Option<String>,
    pub rawDescription: String,
    pub rawDisplayName: String
}


// Event data
#[derive(Deserialize, Debug, Clone, Serialize )]
pub struct ClientApiEventHeader{
    pub Events: Vec<ClientApiEvent>
}

#[derive(Deserialize, Debug, Clone, Serialize )]
pub struct ClientApiEvent{
    pub EventID: i32,
    pub EventName: String,
    pub EventTime: f32,
    pub Assisters: Option<Vec<String>>,
    pub KillerName: Option<String>,
    pub VictimName: Option<String>,
    pub Recipient: Option<String>,
    pub TurretKilled: Option<String>,
    pub InhibKilled: Option<String>,
    pub Stolen: Option<String>,
    pub DragonType: Option<String>,
    pub KillStreak: Option<i32>
}

// Public Data


// Champion Data
#[derive(Deserialize, Debug, Serialize )]
pub struct PublicApiChampions{
    pub data: HashMap<String, PublicApiChampion>
}

#[derive(Deserialize, Debug, Serialize )]
pub struct PublicApiChampion{
    pub key: String,
    pub id: String,
    pub name: String,
    pub title: String,
    pub blurb: String,
    pub info: PublicApiChampionInfo,
    pub image: PublicApiChampionImageData,
    pub tags: Vec<String>,
    pub partype: String,
    pub stats: PublicApiChampionStats
}

#[derive(Deserialize, Debug, Serialize )]
pub struct PublicApiChampionInfo{
    pub attack: f32,
    pub defense: f32,
    pub magic: f32,
    pub difficulty: f32
}

#[derive(Deserialize, Debug, Serialize )]
pub struct PublicApiChampionStats{
    pub hp: f32,
    pub hpperlevel: f32,
    pub mp: f32,
    pub mpperlevel: f32,
    pub movespeed: f32,
    pub armor: f32,
    pub armorperlevel: f32,
    pub spellblock: f32,
    pub spellblockperlevel: f32,
    pub attackrange: f32,
    pub hpregen: f32,
    pub hpregenperlevel: f32,
    pub mpregen: f32,
    pub mpregenperlevel: f32,
    pub crit: f32,
    pub critperlevel: f32,
    pub attackdamage: f32,
    pub attackdamageperlevel: f32,
    pub attackspeed: f32,
    pub attackspeedperlevel: f32,
}

#[derive(Deserialize, Debug, Serialize )]
pub struct PublicApiChampionImageData{
    pub full: String,
    pub sprite: String,
    pub group: String,
    pub x: i32,
    pub y: i32,
    pub w: i16,
    pub h: i16
}


// PlayRate Data
#[derive(Deserialize, Debug, Default, Serialize )]
pub struct PublicApiPlayRates{
    pub data: HashMap<String, HashMap<String, PublicApiPlayRateLane>>
}

#[derive(Deserialize, Debug, Serialize )]
pub struct PublicApiPlayRateLane{
    pub playRate: f32 
}