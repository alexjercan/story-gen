use std::fmt::Display;

pub type PipelineResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    StoryError(String),
    ParseError(String),
    AudioError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::StoryError(msg) => format!("StoryError: {}", msg),
            Error::ParseError(msg) => format!("ParseError: {}", msg),
            Error::AudioError(msg) => format!("AudioError: {}", msg),
        };

        write!(f, "{}", msg)
    }
}
