use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    ParserError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::ParserError(msg) => write!(f, "ParserError: {}", msg),
        }
    }
}
