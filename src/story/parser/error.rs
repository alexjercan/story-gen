pub use story_gen_parser::error::Error;

pub type StoryParserResult<T> = Result<T, Error>;
