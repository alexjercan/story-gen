mod components;
mod layout;
mod resources;
mod systems;

use crate::AppState;
use bevy::prelude::*;
pub use resources::SelectedStory;
use systems::*;

pub struct SelectionMenuPlugin;

impl Plugin for SelectionMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedStory>()
            .add_systems(OnEnter(AppState::SelectionMenu), spawn_selection_menu)
            .add_systems(OnExit(AppState::SelectionMenu), despawn_selection_menu)
            .add_systems(
                Update,
                (
                    interact_with_selection_button,
                    update_system_text,
                    update_voices_text,
                    interact_with_back_button,
                    update_next_visibility,
                    interact_with_next_button,
                )
                    .run_if(in_state(AppState::SelectionMenu)),
            );
    }
}
