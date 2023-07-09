use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum AudioError {
    UnknownName(String),
    FakeYouError(String),
}

impl Display for AudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            AudioError::UnknownName(name) => format!("UnknownName: {}", name),
            AudioError::FakeYouError(e) => format!("FakeYouError: {}", e),
        };

        write!(f, "{}", msg)
    }
}

// TODO: Maybe better errors?
pub type PipelineResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    StoryError(String),
    ParseError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::StoryError(msg) => format!("StoryError: {}", msg),
            Error::ParseError(msg) => format!("ParseError: {}", msg),
        };

        write!(f, "{}", msg)
    }
}
