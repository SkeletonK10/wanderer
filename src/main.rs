use std::fs;

use serde::{Deserialize, Serialize};

const SAVE_PATH: &str = "./save.json";

const HELP: &str = "wanderer — 세계를 떠도는 텍스트 어드벤처

사용법: wd <명령어> [인자]

명령어:
  look        주변을 둘러봅니다
  go <방향>   그 방향을 통해 이동합니다 (예: wd go door)
  help        이 도움말을 보여줍니다";

enum ParseError {
    InvalidUsage(&'static str),
    UnknownCommand(String),
}

#[derive(Deserialize, Serialize)]
struct State {
    room: String,
}

enum Command {
    Go(String),
    Look,
    Help,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let words: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let mut save = read_save();
    match parse(words.as_slice()) {
        Ok(Command::Go(dir)) => match next_room(&save.room, &dir) {
            Some(next) => {
                println!("{}", describe(&next));
                save.room = next;
                write_save(&save);
            }
            None => {
                println!("이동할 수 없습니다.");
            }
        },
        Ok(Command::Look) => println!("{}", describe(&save.room)),
        Ok(Command::Help) => println!("{}", HELP),
        Err(ParseError::UnknownCommand(cmd)) => {
            println!("알 수 없는 명령입니다: {cmd}\nwd help 를 입력해 보세요.")
        }
        Err(ParseError::InvalidUsage(cmd)) => {
            println!("사용법이 올바르지 않습니다: {cmd}\nwd help 를 입력해 보세요.")
        }
    }
}

fn parse(words: &[&str]) -> Result<Command, ParseError> {
    match words {
        [] => Ok(Command::Help),
        ["go", dir] => Ok(Command::Go(dir.to_string())),
        ["go", ..] => Err(ParseError::InvalidUsage("go")),
        ["look"] => Ok(Command::Look),
        ["look", ..] => Err(ParseError::InvalidUsage("look")),
        ["help", ..] => Ok(Command::Help),
        [unknown, ..] => Err(ParseError::UnknownCommand(unknown.to_string())),
    }
}

fn read_save() -> State {
    let default_state = State {
        room: "home".to_string(),
    };
    let read = fs::read_to_string(SAVE_PATH);
    match read {
        Ok(s) => serde_json::from_str(s.as_str()).unwrap(),
        Err(_) => default_state,
    }
}

fn write_save(state: &State) {
    fs::write(SAVE_PATH, serde_json::to_string_pretty(state).unwrap()).unwrap();
}

fn describe(room: &str) -> String {
    match room {
        "home" => "내 집!".to_string(),
        "street" => "시끌벅적한 길.".to_string(),
        _ => "아무것도 보이지 않는다. (세이브 파일 오류)".to_string(),
    }
}

fn next_room(room: &str, dir: &str) -> Option<String> {
    match (room, dir) {
        ("home", "door") => Some("street".to_string()),
        _ => None,
    }
}
