use bevy::prelude::*;
use crate::pipeline::ActionStory;

#[derive(Event, Debug)]
pub struct InputActionStoryEvent(pub ActionStory);

#[derive(Event, Debug)]
pub struct CreatedEndOfStoryEvent;
