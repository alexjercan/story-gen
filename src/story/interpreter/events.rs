use super::StoryAction;
use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct InputActionStoryEvent(pub StoryAction);
