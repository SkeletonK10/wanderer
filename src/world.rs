use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const WORLD_TOML: &str = include_str!("../worlds/fantasy.toml");

pub const DEFAULT_ROOM: &str = "home";

#[derive(Deserialize, Serialize)]
pub struct Room {
    description: String,
    exits: HashMap<String, String>,
}

#[derive(Deserialize, Serialize)]
pub struct World {
    rooms: HashMap<String, Room>,
}

pub fn load() -> World {
    match toml::from_str::<World>(WORLD_TOML) {
        Ok(world) => world,
        Err(e) => panic!("world.toml 파싱 실패:\n{e}"),
    }
}

impl World {
    pub fn describe(&self, room_id: &str) -> String {
        match self.rooms.get(room_id) {
            Some(room) => room.description.clone(),
            None => "아무것도 보이지 않는다.".to_string(),
        }
    }

    pub fn next_room(&self, room_id: &str, dir: &str) -> Option<String> {
        let room = self.rooms.get(room_id)?;
        room.exits.get(dir).cloned()
    }
}
