use crate::loader::StoryAsset;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SelectedStory(pub Option<StoryAsset>);
