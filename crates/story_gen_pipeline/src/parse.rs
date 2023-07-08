use story_gen_parser::{actions, Action};

use crate::{Error, PipelineResult};

pub(crate) fn parse_story(story: &str) -> PipelineResult<Vec<Action>> {
    actions(story)
        .map(|(_, actions)| actions)
        .map_err(|e| Error::ParseError(e.to_string()))
}

