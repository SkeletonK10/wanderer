use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match &args[1..] {
        [] => println!("사용법: wd <명령어>"),
        [cmd] => println!("명령: {}", cmd),
        [cmd, rest @ ..] => println!("명령: {}, 인자: {:?}", cmd, rest),
    }
}

fn read_save() -> String {
    let path = "./save.txt";
    let default_save = "home";
    match fs::read_to_string(path) {
        Err(_) => default_save.to_string(),
        Ok(save) => save.trim().to_string(),
    }
}

fn describe(room: &str) -> String {
    match room {
        "home" => "내 집!".to_string(),
        "void" => "안개가 일렁거린다.".to_string(),
        _ => "아무것도 보이지 않는다. (세이브 파일 오류)".to_string(),
    }
}

fn next_room(room: &str, dir: &str) -> Option<String> {
    match (room, dir) {
        ("home", "door") => Some("void".to_string()),
        _ => None,
    }
}
