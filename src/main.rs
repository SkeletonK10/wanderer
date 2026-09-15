use std::fs;

const SAVE_PATH: &str = "./save.txt";

enum CommandParseError {
    EmptyCommand,
    UnknownCommand(String),
}

enum Command {
    Go(String),
    Look,
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
                println!("그 방향으로 이동할 수 없습니다.");
            }
        },
        Ok(Command::Look) => println!("{}", describe(&room)),
        Err(CommandParseError::EmptyCommand) => {
            println!("명령을 입력해 주세요. (wd go [dir] / wd look)")
        }
        Err(CommandParseError::UnknownCommand(cmd)) => println!("알 수 없는 명령입니다: {}", cmd),
    }
}

fn parse(words: &[&str]) -> Result<Command, CommandParseError> {
    match words {
        [] => Err(CommandParseError::EmptyCommand),
        ["go", dir, ..] => Ok(Command::Go(dir.to_string())),
        ["look", ..] => Ok(Command::Look),
        [unknown, ..] => Err(CommandParseError::UnknownCommand(unknown.to_string())),
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
