pub type StoryGPTResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    StoryError(String),
}

impl From<openai_api_rust::Error> for Error {
    fn from(err: openai_api_rust::Error) -> Self {
        Error::StoryError(err.to_string())
    }
}

