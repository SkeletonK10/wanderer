use crate::game::State;
use std::fs;

const SAVE_PATH: &str = "./save.json";

pub enum SaveError {
    Io(std::io::Error),
    Corrupt(serde_json::Error),
    Serialize(serde_json::Error),
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SaveError::Io(err) => write!(
                f,
                "세이브 파일에 접근할 수 없습니다: {SAVE_PATH}\n    {err}"
            ),
            SaveError::Corrupt(err) => {
                write!(
                    f,
                    "세이브 파일이 손상되어 불러올 수 없습니다: {SAVE_PATH}\n    {err}\n삭제하면 새 게임으로 시작하지만 진행 상황은 사라집니다."
                )
            }
            SaveError::Serialize(err) => write!(
                f,
                "진행 상황을 저장 형식으로 바꾸지 못했습니다. 프로그램 버그입니다.\n    {err}"
            ),
        }
    }
}

pub fn read_save() -> Result<State, SaveError> {
    match fs::read_to_string(SAVE_PATH) {
        Ok(json) => serde_json::from_str(&json).map_err(SaveError::Corrupt),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(State::default()),
        Err(err) => Err(SaveError::Io(err)),
    }
}

pub fn write_save(state: &State) -> Result<(), SaveError> {
    match serde_json::to_string_pretty(state) {
        Ok(json) => fs::write(SAVE_PATH, json).map_err(SaveError::Io),
        Err(err) => Err(SaveError::Serialize(err)),
    }
}
