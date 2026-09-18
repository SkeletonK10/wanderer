use crate::game::Command;

pub enum Input {
    Empty,
    Quit,
    Help,
    Game(Command),
}

pub enum ParseError {
    InvalidUsage(&'static str),
    UnknownCommand(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InvalidUsage(cmd) => {
                write!(f, "올바르지 않은 명령입니다: {cmd}\nhelp 를 입력해 보세요.")
            }
            ParseError::UnknownCommand(cmd) => {
                write!(f, "알 수 없는 명령입니다: {cmd}\nhelp 를 입력해 보세요.")
            }
        }
    }
}

pub fn parse(words: &[&str]) -> Result<Input, ParseError> {
    match words {
        [] => Ok(Input::Empty),
        ["go"] => Err(ParseError::InvalidUsage("go")),
        ["go", rest @ ..] => Ok(Input::Game(Command::Go(rest.join(" ")))),
        ["look"] => Ok(Input::Game(Command::Look)),
        ["look", ..] => Err(ParseError::InvalidUsage("look")),
        ["help", ..] => Ok(Input::Help),
        ["quit", ..] => Ok(Input::Quit),
        [unknown, ..] => Err(ParseError::UnknownCommand(unknown.to_string())),
    }
}
