use std::fs;

const SAVE_PATH: &str = "./save.txt";

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

enum Command {
    Go(String),
    Look,
    Help,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let words: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let save = read_save();
    let room = save;
    match parse(words.as_slice()) {
        Ok(Command::Go(dir)) => match next_room(&room, &dir) {
            Some(next) => {
                println!("{}", describe(&next));
                write_save(&next);
            }
            None => {
                println!("이동할 수 없습니다.");
            }
        },
        Ok(Command::Look) => println!("{}", describe(&room)),
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

fn read_save() -> String {
    let default_save = "home";
    match fs::read_to_string(SAVE_PATH) {
        Err(_) => default_save.to_string(),
        Ok(save) => save.trim().to_string(),
    }
}

fn write_save(save: &str) {
    fs::write(SAVE_PATH, save).unwrap();
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
