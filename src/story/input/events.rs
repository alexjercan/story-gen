use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct InputTextEvent(pub String);
