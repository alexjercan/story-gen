use super::ActionStory;
use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct InputPromptEvent(pub String);

#[derive(Event, Debug)]
pub struct CreatedActionStoryEvent(pub ActionStory);
