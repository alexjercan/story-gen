mod components;
mod layout;
mod systems;

use bevy::prelude::*;

use systems::*;

use crate::AppState;

use super::StoryState;

pub struct StoryInputPlugin;

impl Plugin for StoryInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<StoryState>()
            .add_systems(OnEnter(AppState::Story), spawn_input_menu)
            .add_systems(OnExit(AppState::Story), despawn_input_menu)
            .add_systems(OnEnter(StoryState::Idle), show_input_menu)
            .add_systems(OnExit(StoryState::Idle), hide_input_menu)
            .add_systems(
                Update,
                (
                    interact_with_input_text,
                    interact_with_continue_button,
                    submit_input_text,
                )
                    .run_if(in_state(AppState::Story).and_then(in_state(StoryState::Idle))),
            );
    }
}
