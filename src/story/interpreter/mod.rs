mod components;
mod events;
mod resources;
mod systems;

use crate::AppState;
use bevy::prelude::*;
pub use events::InputActionStoryEvent;
use resources::*;
use systems::*;

pub type TTSResult<T> = Result<T, String>;

#[derive(Debug, Clone)]
pub struct SayAction {
    pub name: String,
    pub text: String,
    pub tts: TTSResult<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct CommentAction {
    pub text: String,
}

#[derive(Debug, Clone)]
pub enum StoryAction {
    Say(SayAction),
    Comment(CommentAction),
}

pub struct InterpreterPlugin;

impl Plugin for InterpreterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputActionStoryEvent>()
            .init_resource::<TTSOptions>()
            .init_resource::<StoryActions>()
            .add_systems(
                Update,
                handle_input_action.run_if(in_state(AppState::Story)),
            )
            .add_systems(
                Update,
                poll_action_loader_task.run_if(in_state(AppState::Story)),
            );
    }
}
