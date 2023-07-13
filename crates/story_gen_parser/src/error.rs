pub type StoryGPTResult<T> = Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    ParserError(String),
}
