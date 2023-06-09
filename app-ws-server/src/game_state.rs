use rand::{seq::IteratorRandom, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, vec};

use crate::player::Player;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum PlayerAction {
    LogIn { player_name: String },
    CollectFood,
    CollectWater,
    CollectWood { draws: u8 },
}

#[derive(Debug, Clone)]
pub struct GameState {
    weathers: Vec<u8>,
    players: HashMap<u8, Player>,
    current_water: u8,
    current_wood: u8,
    current_food: u8,
    current_player: u8,
    turn_count: u8,
    started: bool,
    storm: u8,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]

pub struct BoardState {
    weather: u8,
    storm: bool,
    players: HashMap<u8, Player>,
    current_water: u8,
    current_wood: u8,
    current_food: u8,
    current_player: u8,
    turn_count: u8,
    started: bool,
}

impl Default for GameState {
    fn default() -> Self {
        let mut weathers = {
            let mut raw: Vec<u8> = vec![0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3];
            let mut results: Vec<u8> = vec![];
            let l = raw.len();
            // Mix the weather cards
            for _ in 0..l {
                let (i, &out) = raw.iter().enumerate().choose(&mut thread_rng()).unwrap();
                raw.remove(i);
                results.push(out);
            }
            results
        };
        let storm_day = 6 + {
            let mut rng = thread_rng();
            rng.gen_range(0..6)
        };
        weathers.insert(storm_day, 2);
        Self {
            weathers,
            players: Default::default(),
            current_water: Default::default(),
            current_wood: Default::default(),
            current_food: Default::default(),
            current_player: Default::default(),
            turn_count: Default::default(),
            started: Default::default(),
            storm: storm_day as u8,
        }
    }
}

impl GameState {
    pub fn to_board_state(&self) -> BoardState {
        BoardState {
            weather: self.weathers.get(self.turn_count as usize).unwrap().clone(),
            storm: self.storm == self.turn_count,
            players: self.players.clone(),
            current_water: self.current_water,
            current_wood: self.current_wood,
            current_food: self.current_food,
            current_player: self.current_player,
            turn_count: self.turn_count,
            started: self.started,
        }
    }

    pub fn draw_wood(&mut self, _player_id: u8, _draws: u8) {
        ()
    }

    pub fn draw_food(&mut self) {
        match rand::thread_rng().gen_range(0.0..=1.0) {
            x if x < 0.5 => self.current_food += 1,
            x if x < 5.0 / 6.0 => self.current_food += 2,
            _ => self.current_food += 3,
        }
    }

    pub fn collect_water(&mut self) {
        self.current_water += self.weathers.get(self.turn_count as usize).unwrap();
    }

    pub fn add_player(&mut self, player_name: String, id: u8) {
        // Can't connect when the game is already started
        if self.started {
            return;
        }
        //Return if the user is already connected
        if let Some(_) = self.players.get(&id) {
            return;
        }
        //Insert new player
        self.players.insert(id, Player::new(player_name));
        if self.players.is_empty() {
            self.current_player = id;
        }
    }

    pub fn perform_action(&mut self, action: PlayerAction, id: u8) {
        println!(
            "Performing action : {} \non current game state.",
            serde_json::to_string_pretty(&action).unwrap()
        );
        match action {
            PlayerAction::LogIn { player_name } => self.add_player(player_name, id),
            PlayerAction::CollectFood => self.draw_food(),
            PlayerAction::CollectWater => self.collect_water(),
            PlayerAction::CollectWood { draws } => self.draw_wood(id, draws),
        }
    }

    pub fn on_player_disconnected(&mut self, id: u8) {
        if let Some(p) = self.players.get_mut(&id) {
            p.connected = false;
        }
    }
}
