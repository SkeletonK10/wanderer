mod game;
mod parse;
mod save;
mod world;

fn main() {
    let world = world::load();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let words: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let state = save::read_save();
    let (new_state, message) = match parse::parse(words.as_slice()) {
        Ok(command) => game::apply(&world, state, command),
        Err(parse::ParseError::UnknownCommand(cmd)) => {
            println!("알 수 없는 명령입니다: {cmd}\nwd help 를 입력해 보세요.");
            return;
        }
        Err(parse::ParseError::InvalidUsage(cmd)) => {
            println!("사용법이 올바르지 않습니다: {cmd}\nwd help 를 입력해 보세요.");
            return;
        }
    };
    save::write_save(&new_state);
    println!("{message}");
}
