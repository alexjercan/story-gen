use super::error::StoryGPTResult;
use bevy::{prelude::*, tasks::Task};

#[derive(Component)]
pub struct StoryGPTLoader(pub Task<StoryGPTResult<String>>);
