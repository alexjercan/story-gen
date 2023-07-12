mod components;
mod layout;
mod systems;

use bevy::prelude::*;

use systems::*;

use crate::AppState;

pub struct OptionsMenuPlugin;

impl Plugin for OptionsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::OptionsMenu), spawn_options_menu)
            .add_systems(OnExit(AppState::OptionsMenu), despawn_options_menu)
            .add_systems(
                Update,
                (interact_with_back_button,).run_if(in_state(AppState::OptionsMenu)),
            );
    }
}
