use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct CreatedTextEvent(pub String);

#[derive(Event, Debug)]
pub struct InputMenuEvent(pub Visibility);
