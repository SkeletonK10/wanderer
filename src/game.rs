use serde::{Deserialize, Serialize};

use crate::world;

#[derive(Deserialize, Serialize)]
pub struct State {
    room: String,
}

pub enum Command {
    Go(String),
    Look,
}

impl Default for State {
    fn default() -> Self {
        State {
            room: world::DEFAULT_ROOM.to_string(),
        }
    }
}

#[must_use]
pub fn apply(world: &world::World, state: State, command: Command) -> (State, String) {
    match command {
        Command::Go(dir) => match world.next_room(&state.room, &dir) {
            Some(next) => {
                let new_state = State {
                    room: next.to_string(),
                    ..state
                };
                let message = world.describe(&new_state.room);
                (new_state, message)
            }
            None => (state, "이동할 수 없습니다!".to_string()),
        },
        Command::Look => {
            let message = world.describe(&state.room);
            (state, message)
        }
    }
}
