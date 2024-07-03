pub mod model;
use std::{
    collections::HashMap,
    time::Duration, fs::File, path::Path,
};
use model::{*};
use rand::seq::IteratorRandom;

use reqwest::Certificate;

impl model::API {
    pub fn new() -> Self{
        let built_client = reqwest::Client::builder()
        .add_root_certificate(Certificate::from_pem(include_bytes!("riotgames.cer")).unwrap())
        .build()
        .expect("Unable to build reqwest client");

        Self {
            version: "".to_owned(),
            debug_mode: false,
            debug_data: None,
            api_client: built_client, 
            in_game: false,
            game_type: Gametype::None,
            active_player: ClientApiActivePlayer::default(),
            events: Vec::new(),
            players: vec![],
            champions: vec![],
            settings: API_SETTINGS { 
                endpoints: API_ENDPOINTS::default()
            }
        }
    }

    pub fn start(&mut self){
        self.version = match self.get_game_version() {
            Ok(version) => version,
            Err(_) => {print!("Unable to retrieve game version"); "13.24.1".to_string()}
        };
        self.set_champions();
        self.load_settings();
        
        self.update();
    }

    pub fn update(&mut self){
        self.in_game = self.is_game_alive();

        if !self.in_game && !self.debug_mode {return;}

        // Get live game data
        let game_data = match self.get_game_data() {
            Ok(data) => data,
            _ => {println!("Invalid Game Data"); return;}
        };


        self.active_player = game_data.activePlayer;
        self.game_type = self.parse_game_type(game_data.gameData);
        self.players = self.parse_players(game_data.allPlayers);
        self.parse_events(game_data.events.Events);
    }

    fn load_settings(&mut self){
        let json_location = "./settings.json";

        let json_file = match File::open(json_location) {
            Ok(file) => file,
            _ => {println!("Settings not found"); return;}
        };

        self.settings = serde_json::from_reader(json_file).expect("Unable to load settings");
    }

    #[tokio::main]
    pub async fn is_game_alive(&self) -> bool {
        let request_url = &self.settings.endpoints.game_status;
        return self.api_client.get(request_url).timeout(Duration::from_millis(100)).send().await.is_ok();
    }

    #[tokio::main]
    pub async fn get_game_version(&mut self) ->  Result<String,Box<dyn std::error::Error>> {
        let request_url = &self.settings.endpoints.game_version;
        let response = self.api_client
        .get(request_url)
        .send()
        .await?;

        let versions:Vec<String> = response.json().await?;

        Ok(versions.get(0).unwrap().to_string())
    }

    #[tokio::main]
    pub async fn get_game_data(&self) ->  Result<ClientApiGameData,Box<dyn std::error::Error>> {
        if self.debug_mode{
            return match self.debug_data.clone() {
                Some(data) => Ok(data),
                None => Err("No debug data".into())
            };
        }

        let request_url = &self.settings.endpoints.game_data;
        let response = self.api_client
        .get(request_url)
        .send()
        .await?;
    
        let game_data:ClientApiGameData = response.json().await?;

        Ok(game_data)
    }
    
    #[tokio::main]
    pub async fn get_champions(&self) ->  Result<PublicApiChampions,Box<dyn std::error::Error>> {
        let request_url = self.settings.endpoints.champions.to_owned().replace("{VERSION}", &self.version);
        let response = self.api_client
        .get(request_url)
        .send()
        .await?;

        let champions:PublicApiChampions = response.json().await?;
        Ok(champions)
    }

    pub fn get_random_champion_by_lane(&self, filter: &Lane ) ->  Champion {
        let random_champion = self.champions.iter().filter(|champion|{
            champion.lanes.contains(&filter) || filter == &Lane::None
        }).choose(&mut rand::thread_rng());

        match random_champion {
            Some(champion) => champion.clone(),
            None => Champion::Default(&self.game_type)
        }
    }

    #[tokio::main]
    pub async fn get_play_rate(&self) ->  Result<PublicApiPlayRates,Box<dyn std::error::Error>> {
        let request_url = &self.settings.endpoints.play_rate;
        let response = self.api_client
        .get(request_url)
        .send()
        .await?;

        let champions:PublicApiPlayRates = response.json().await?;
        Ok(champions)
    }
    
    // Snapshots are files or snapshots if you will, of old game data that can be used to test the parser with known good data
    pub fn set_snapshot(&mut self, game_mode: Gametype){
        let data_location = match game_mode {
            Gametype::TFT => "./snapshots/tft.json",
            Gametype::Arena => "./snapshots/arena.json",
            Gametype::ARAM => "./snapshots/aram.json",
            _ => "./snapshots/classic.json"
        };
        
        let json_location = Path::new(data_location);

        let json_file = match File::open(json_location) {
            Ok(file) => file,
            _ => {
                println!("Snapshot not found"); 
                return;
            }
        };

        self.debug_data = match serde_json::from_reader(json_file) {
            Ok(data) => Some(data),
            _ => None
        };

        self.debug_mode = true;
    }
    
    pub fn set_champions(&mut self){
        let champion_data = match self.get_champions() {
            Ok(data) => data,
            Err(_) => {
                println!("Champion data couldn't be loaded."); 
                return;
            }           
        };

        let play_rate_data = match self.get_play_rate() {
            Ok(data) => data,
            Err(_) => {
                println!("Play rate data couldn't be loaded, lane filtering will be unavailable."); 
                PublicApiPlayRates::default()
            }           
        };

        let mut parsed_champions: Vec<Champion> = vec![];

        for (_, champion) in champion_data.data{
            let mut champion_lanes: Vec<Lane> = vec![];
            // Parse champion lane data
            let default_rate:HashMap<String, PublicApiPlayRateLane> = HashMap::new();
            let champion_play_rates = &play_rate_data.data.get(&champion.key).unwrap_or(&default_rate);
            for (lane, play_rate) in champion_play_rates.iter(){
                if play_rate.playRate > 0.{
                    champion_lanes.push(match lane.as_str() {
                        "TOP" => Lane::Top,
                        "JUNGLE" => Lane::Jungle,
                        "MIDDLE" => Lane::Middle,
                        "BOTTOM" => Lane::Bottom,
                        "UTILITY" => Lane::Support,
                        _ => Lane::None
                    });
                }
            }

            parsed_champions.push(Champion { 
                name: champion.name, 
                image: format!("champion/{}.png", &champion.id),
                id: champion.id,
                lanes: champion_lanes 
            });
        }

        self.champions = parsed_champions;
    }

    pub fn parse_game_type(&self, game_type_data: ClientApiGameTypeData) -> Gametype{
        match game_type_data.gameMode.as_str() {
            "Classic" => Gametype::Normal,
            "ARAM" => Gametype::ARAM,
            "CHERRY" => Gametype::Arena,
            "TFT" => Gametype::TFT,
            _ => Gametype::None
        }
    }

    pub fn parse_players(&mut self, players: Vec<ClientApiPlayer>) -> Vec<Player>{
        let mut parsed_players: Vec<Player> = vec![];
        for player in players.iter(){

            let player_champion:Champion = self.champions.iter()
                .find(|champion|{champion.name == player.championName})
                .unwrap_or(&Champion::Default(&self.game_type))
                .clone();

            parsed_players.push(Player{
                name: player.summonerName.clone(),
                champion: player_champion.clone(),
                lane: match player.position.as_str() {
                    "TOP" => Lane::Top,
                    "JUNGLE" => Lane::Jungle,
                    "MIDDLE" => Lane::Middle,
                    "BOTTOM" => Lane::Bottom,
                    "UTILITY" => Lane::Support,
                    _ => Lane::None
                },
                team: match  player.team.as_str(){
                    "ORDER" => Team::Blue,
                    _ => Team::Red
                },
                ancestor: player.clone()
            });
         }
         parsed_players
    }

    pub fn parse_events(&mut self, events: Vec<ClientApiEvent>){
        self.events = Vec::new();
        for event in events.iter(){
            match event.EventName.as_str() {
                "TurretKilled" | "InhibKilled" => self.parse_structure(event),
                "DragonKill" | "HeraldKill" | "BaronKill" => self.parse_objective(event),
                "Multikill"=> self.parse_multikill(event),
                "FirstBlood"=> self.parse_firstblood(event, events.clone()), 
                _ => ()
            }
        }
    }

    pub fn get_event_aggressor(&self, event: &ClientApiEvent) -> Option<&Player>{
        match &event.KillerName {
            Some(killer) => self.players.iter().find(|player: &&Player| &player.name == killer),
            _ => None
        }        
    }

    pub fn get_event_target_player(&self, event: &ClientApiEvent) -> Option<&Player>{
        let event_target: &String = match (&event.VictimName, &event.Recipient) {
            (Some(target), None) | (None, Some(target)) => target,
            (Some(target1), Some(target2)) => {
                if target1 != target2 {
                    println!("Unable to determine event target, event has both VictimName and Recipient fields which don't match eachother.");
                    return None;
                }

                target1
            },
            _ => {return None;}
        };

        self.players.iter().find(|player: &&Player| &player.name == event_target)
    }

    pub fn parse_structure(&mut self, event: &ClientApiEvent){

        let structure = match (event.TurretKilled.as_deref(), event.InhibKilled.as_deref()) {
            (Some("Obelisk"), None) => {return;},
            (Some(structure), None) | (None, Some(structure)) => structure,
            _ => {
                println!("Turret or inhibitor unspecified, cannot parse event.");
                return;
            }
        };

        let structure_data:Vec<&str> = structure.split("_").collect();

        let team = match structure_data.get(1) {
            Some(&"T2") => Team::Red,
            _ => Team::Blue
        };

        let lane = match structure_data.get(2) {
            Some(&"L") | Some(&"L1") => Lane::Top,
            Some(&"C") | Some(&"C1") => Lane::Middle,
            Some(&"R") | Some(&"R1") => Lane::Bottom,
            _ => Lane::None
        };

        self.events.push(match structure_data.get(0) {
            Some(&"Barracks") => Events::Inhibitor { team: team, lane: lane },
            Some(&"Turret") =>{ match structure_data.get(0) {
                Some(&"01") | Some(&"02") if lane == Lane::Middle => Events::NexusTurret { team: team },
                _ => Events::Turret { team: team, lane: lane }
            }},
            _ => Events::Turret { team: team, lane: lane }
        });
    }

    pub fn parse_objective(&mut self, event: &ClientApiEvent){
        let killer = match self.get_event_aggressor(event) {
            Some(player) => player,
            _ => {
                println!("Unable to determine event aggressor.");
                return;
            }
        };

        let team: Team = killer.team;

        let stolen: bool = match event.Stolen.clone().as_deref() {
            Some("True") => false,
            _ => false
        };

        self.events.push(match event.EventName.as_str() {
            "DragonKill" => {match event.DragonType.clone().as_deref(){
                Some("Elder") => Events::Elder { team: team, stolen: stolen},
                _ => Events::Dragon { team: team, stolen: stolen}
            }},
            "HeraldKill" => Events::Herald { team: team, stolen: stolen},
            "BaronKill" => Events::Baron { team: team, stolen: stolen},
            _ => Events::Dragon { team: team, stolen: stolen}
        });
    }

    pub fn parse_multikill(&mut self, event: &ClientApiEvent){
        let killer = match self.get_event_aggressor(event) {
            Some(player) => player,
            _ => {
                println!("Unable to determine event perpetrator");
                return;
            }
        };

        match event.KillStreak {
            Some(5) => self.events.push(
                Events::Penta { 
                    team: killer.team, 
                    player: killer.clone() 
                }
            ),
            _ => ()
        }
    }

    pub fn parse_firstblood(&mut self, event: &ClientApiEvent , event_log: Vec<ClientApiEvent>){

        let last_event = match event_log.iter().find(|evnt| evnt.EventID == event.EventID - 1) {
            Some(event) => event,
            _ => {return;}
        };

        match self.get_event_target_player(last_event) {
            Some(victim) => self.events.push(
                Events::FirstBlood { 
                    team: victim.team.flip(), 
                    player: victim.clone()
                }
            ),
            _ => ()
        };

    }
}