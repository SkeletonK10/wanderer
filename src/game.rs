use serde::{Deserialize, Serialize};

use crate::world;

const HELP: &str = "wanderer — 세계를 떠도는 텍스트 어드벤처

사용법: wd <명령어> [인자]

명령어:
  look        주변을 둘러봅니다
  go <방향>   그 방향을 통해 이동합니다 (예: wd go door)
  help        이 도움말을 보여줍니다";

#[derive(Deserialize, Serialize)]
pub struct State {
    room: String,
}

pub enum Command {
    Go(String),
    Look,
    Help,
}

impl Default for State {
    fn default() -> Self {
        State {
            room: world::DEFAULT_ROOM.to_string(),
        }
    }
}

#[must_use]
pub fn apply(state: State, command: Command) -> (State, String) {
    match command {
        Command::Go(dir) => match world::next_room(&state.room, &dir) {
            Some(next) => {
                let new_state = State {
                    room: next,
                    ..state
                };
                let message = world::describe(&new_state.room);
                (new_state, message)
            }
            None => (state, "이동할 수 없습니다!".to_string()),
        },
        Command::Look => {
            let message = world::describe(&state.room);
            (state, message)
        }
        Command::Help => (state, HELP.to_string()),
    }
}
