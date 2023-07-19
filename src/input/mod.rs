mod components;
mod events;
mod layout;
mod resources;
mod systems;

use crate::AppState;
use bevy::prelude::*;
pub use events::{CreatedTextEvent, InputMenuEvent};
use resources::*;
use systems::*;

pub struct StoryInputPlugin;

impl Plugin for StoryInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputVisible>()
            .add_event::<CreatedTextEvent>()
            .add_event::<InputMenuEvent>()
            .add_systems(OnEnter(AppState::Story), spawn_input_menu)
            .add_systems(OnExit(AppState::Story), despawn_input_menu)
            .add_systems(
                Update,
                (
                    show_input_menu,
                    handle_input_menu,
                    interact_with_input_text,
                    submit_input_text,
                )
                    .run_if(in_state(AppState::Story)),
            );
    }
}
