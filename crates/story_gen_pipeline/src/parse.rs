use story_gen_parser::{actions, Action};

use crate::{Error, PipelineResult};

#[derive(Debug, Default)]
pub(crate) struct Parser {
    input: String,
}

impl Parser {
    pub(crate) fn new() -> Self {
        Self {
            input: String::new(),
        }
    }

    pub(crate) fn clear(&mut self) {
        self.input.clear();
    }

    pub(crate) fn parse(&mut self, story: &str) -> PipelineResult<Vec<Action>> {
        self.input.push_str(story);
        let (input, actions) = actions(&self.input).map_err(|e| Error::ParseError(e.to_string()))?;
        self.input = input.to_string();

        Ok(actions)
    }
}
