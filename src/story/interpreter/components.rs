use super::StoryAction;
use bevy::{prelude::*, tasks::Task};

#[derive(Component)]
pub struct InterpreterActionLoader(pub Task<StoryAction>);
