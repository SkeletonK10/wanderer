use crate::game::State;
use std::fs;

const SAVE_PATH: &str = "./save.json";

pub fn read_save() -> State {
    let read = fs::read_to_string(SAVE_PATH);
    match read {
        Ok(s) => serde_json::from_str(&s).unwrap(),
        Err(_) => State::default(),
    }
}

pub fn write_save(state: &State) {
    fs::write(SAVE_PATH, serde_json::to_string_pretty(state).unwrap()).unwrap();
}
