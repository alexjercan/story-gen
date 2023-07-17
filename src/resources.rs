use bevy::prelude::*;
use crate::loader::StoryAsset;

#[derive(Resource, Default)]
pub struct Stories {
    pub stories: Vec<Handle<StoryAsset>>,
}
