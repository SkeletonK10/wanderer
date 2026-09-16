use crate::game::Command;

pub enum ParseError {
    InvalidUsage(&'static str),
    UnknownCommand(String),
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
