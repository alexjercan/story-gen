use crate::loader::StoryAsset;
use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct Stories {
    pub stories: Vec<Handle<StoryAsset>>,
}
