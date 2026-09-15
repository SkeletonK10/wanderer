use std::fs;

const SAVE_PATH: &str = "./save.txt";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    match command.as_slice() {
        [] => println!("사용법: wd <명령어>"),
        _ => println!("사용법: wd <명령어>"),
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
