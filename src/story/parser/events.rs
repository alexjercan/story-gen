use bevy::prelude::*;
use story_gen_parser::Action;

#[derive(Event, Debug)]
pub struct InputActionEvent(pub Action);
