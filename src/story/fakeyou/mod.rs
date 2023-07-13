mod systems;

use crate::AppState;
use bevy::prelude::*;
use systems::*;

pub struct FakeYouPlugin;

impl Plugin for FakeYouPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_input_action.run_if(in_state(AppState::Story)));
    }
}
