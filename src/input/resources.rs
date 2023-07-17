use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct InputVisible {
    pub visible: Visibility,
}
