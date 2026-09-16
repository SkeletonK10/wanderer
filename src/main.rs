use std::process::ExitCode;

mod game;
mod parse;
mod save;
mod world;

fn main() -> ExitCode {
    let world = world::load();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let words: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let command = match parse::parse(words.as_slice()) {
        Ok(command) => command,
        Err(e) => {
            eprintln!("wd: {e}");
            return ExitCode::FAILURE;
        }
    };
    let state = match save::read_save() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("wd: {e}");
            return ExitCode::FAILURE;
        }
    };
    let (new_state, message) = game::apply(&world, state, command);
    println!("{message}");
    if let Err(e) = save::write_save(&new_state) {
        eprintln!("wd: {e}");
        return ExitCode::FAILURE;
    };
    ExitCode::SUCCESS
}
