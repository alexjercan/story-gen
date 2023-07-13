mod components;
mod error;
mod events;
mod systems;

use crate::AppState;
use bevy::prelude::*;
pub use events::InputActionEvent;
use systems::*;

pub struct ParserPlugin;

impl Plugin for ParserPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputActionEvent>()
            .add_systems(Update, handle_input_story.run_if(in_state(AppState::Story)))
            .add_systems(
                Update,
                poll_story_loader_task.run_if(in_state(AppState::Story)),
            );
    }
}
