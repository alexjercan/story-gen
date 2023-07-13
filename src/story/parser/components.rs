use super::error::StoryParserResult;
use bevy::{prelude::*, tasks::Task};
use story_gen_parser::Action;

#[derive(Component)]
pub struct StoryParserLoader(pub Task<StoryParserResult<Vec<Action>>>);
