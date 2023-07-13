mod components;
mod events;
mod layout;
mod systems;

use super::StoryState;
use crate::AppState;
use bevy::prelude::*;
pub use events::InputTextEvent;
use systems::*;

pub struct StoryInputPlugin;

impl Plugin for StoryInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputTextEvent>()
            .add_systems(OnEnter(AppState::Story), spawn_input_menu)
            .add_systems(OnExit(AppState::Story), despawn_input_menu)
            .add_systems(OnEnter(StoryState::Idle), show_input_menu)
            .add_systems(OnExit(StoryState::Idle), hide_input_menu)
            .add_systems(
                Update,
                (
                    interact_with_input_text,
                    submit_input_text,
                )
                    .run_if(in_state(AppState::Story).and_then(in_state(StoryState::Idle))),
            );
    }
}
