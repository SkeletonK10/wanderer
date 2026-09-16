use crate::game::Command;

pub enum ParseError {
    InvalidUsage(&'static str),
    UnknownCommand(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InvalidUsage(cmd) => {
                write!(
                    f,
                    "올바르지 않은 명령입니다: {cmd}\nwd help 를 입력해 보세요."
                )
            }
            ParseError::UnknownCommand(cmd) => {
                write!(f, "알 수 없는 명령입니다: {cmd}\nwd help 를 입력해 보세요.")
            }
        }
    }
}

pub fn parse(words: &[&str]) -> Result<Command, ParseError> {
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
