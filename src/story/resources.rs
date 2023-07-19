use std::collections::VecDeque;
use bevy::prelude::*;
use crate::pipeline::ActionStory;

#[derive(Default, Resource, Deref, DerefMut)]
pub struct StoryActions(pub VecDeque<ActionStory>);
