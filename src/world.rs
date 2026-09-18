use crate::SEPARATOR;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};

const WORLD_TOML: &str = include_str!("../worlds/fantasy.toml");

pub const DEFAULT_ROOM: &str = "home";

#[derive(Deserialize)]
pub struct Room {
    description: String,
    exits: BTreeMap<String, String>,
}

#[derive(Deserialize)]
pub struct World {
    rooms: HashMap<String, Room>,
}

pub(crate) fn from_toml(src: &str) -> Result<World, toml::de::Error> {
    toml::from_str(src)
}

pub fn load() -> World {
    match from_toml(WORLD_TOML) {
        Ok(world) => {
            world.validate();
            world
        }
        Err(e) => panic!("{e}"),
    }
}

impl World {
    fn validate(&self) {
        if !self.rooms.contains_key(DEFAULT_ROOM) {
            panic!("No default room")
        }
        for (id, room) in &self.rooms {
            for (exit_id, exit) in &room.exits {
                if !(&self.rooms.contains_key(exit)) {
                    panic!("exit to nowhere : {id} to {exit_id}")
                }
            }
        }
    }
    pub fn describe(&self, room_id: &str) -> String {
        match self.rooms.get(room_id) {
            Some(room) => {
                let desc = &room.description;
                let exit_str = room
                    .exits
                    .keys()
                    .map(|s| format!("- {s}"))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("{desc}\n{SEPARATOR}\n출구 목록:\n{exit_str}")
            }
            None => "아무것도 보이지 않는다.".to_string(),
        }
    }

    pub fn next_room(&self, room_id: &str, dir: &str) -> Option<&str> {
        let room = self.rooms.get(room_id)?;
        room.exits.get(dir).map(String::as_str)
    }
}
