use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct CreatedTextEvent(pub String);
