pub type PipelineResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    StoryError(String),
    ParseError(String),
    AudioError(String),
}
