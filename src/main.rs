use crate::parse::Input;
use std::{io::Write, process::ExitCode};

mod game;
mod parse;
mod save;
mod world;

const HELP: &str = "wanderer — 세계를 떠도는 텍스트 어드벤처

명령어:
  look        주변을 둘러봅니다.
  go <출구>    그 방향을 통해 이동합니다. (예: go street)
  help        이 도움말을 보여줍니다.
  quit        이 게임을 종료합니다.";

const SEPARATOR: &str = "-----------------------------------------------------------------";

fn main() -> ExitCode {
    let world = world::load();
    let mut state = match save::read_save() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("wd: {e}");
            return ExitCode::FAILURE;
        }
    };
    loop {
        print!("> ");
        let _ = std::io::stdout().flush();
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(0) => {
                println!();
                break;
            }
            Ok(_) => (),
            Err(e) => {
                eprintln!("{e}");
                return ExitCode::FAILURE;
            }
        }
        let word_vec = line.split_whitespace().collect::<Vec<_>>();
        let command = match parse::parse(&word_vec) {
            Ok(Input::Empty) => continue,
            Ok(Input::Quit) => break,
            Ok(Input::Help) => {
                show(HELP);
                continue;
            }
            Ok(Input::Game(command)) => command,
            Err(e) => {
                show(&e.to_string());
                continue;
            }
        };
        let (new_state, message) = game::apply(&world, state, command);
        show(&message);
        if let Err(e) = save::write_save(&new_state) {
            eprintln!("wd: {e}");
            return ExitCode::FAILURE;
        };
        state = new_state;
    }

    ExitCode::SUCCESS
}

fn show(text: &str) {
    println!("\n{SEPARATOR}\n{text}\n{SEPARATOR}\n");
}
