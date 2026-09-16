pub const DEFAULT_ROOM: &str = "home";

pub fn describe(room: &str) -> String {
    match room {
        "home" => "내 집!".to_string(),
        "street" => "시끌벅적한 길.".to_string(),
        _ => "아무것도 보이지 않는다. (세이브 파일 오류)".to_string(),
    }
}

pub fn next_room(room: &str, dir: &str) -> Option<String> {
    match (room, dir) {
        ("home", "door") => Some("street".to_string()),
        _ => None,
    }
}
