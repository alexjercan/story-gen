mod events;
mod components;
mod error;
mod resources;
mod systems;

use crate::AppState;

use self::systems::*;
use super::StoryState;
use bevy::prelude::*;
use resources::*;
pub use events::InputStoryEvent;

pub struct ChatGPTPlugin;

impl Plugin for ChatGPTPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputStoryEvent>()
            .init_resource::<StoryChatBody>()
            .init_resource::<StoryChatAuth>()
            .add_systems(Update, handle_input_text.run_if(in_state(AppState::Story)))
            .add_systems(
                Update,
                poll_story_loader_task
                    .run_if(in_state(AppState::Story).and_then(in_state(StoryState::Simulation))),
            );
    }
}
