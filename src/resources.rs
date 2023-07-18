use crate::loader::StoryAsset;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Stories {
    pub stories: Vec<Handle<StoryAsset>>,
}
