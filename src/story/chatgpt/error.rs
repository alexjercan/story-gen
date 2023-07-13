#[derive(Debug)]
pub enum Error {
    StoryError(String),
}

pub type StoryGPTResult<T> = Result<T, Error>;
