mod events;
mod resources;
mod systems;

use bevy::prelude::*;
use chatgpt::*;
pub use events::{CreatedActionStoryEvent, InputPromptEvent};
use fakeyou::*;
use parser::*;
use resources::*;
use systems::*;

#[derive(Debug, Clone)]
pub enum ActionStory {
    Say(String, String, Option<Handle<AudioSource>>),
    Comment(String),
}

pub struct PipelinePlugin;

impl Plugin for PipelinePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActionsQueue>()
            .init_resource::<SayQueue>()
            .add_event::<InputPromptEvent>()
            .add_event::<CreatedActionStoryEvent>()
            .add_plugins((ChatGPTPlugin, ParserPlugin, FakeYouPlugin))
            .add_systems(
                Update,
                (
                    handle_created_text,
                    handle_created_story,
                    handle_created_actions,
                    handle_created_tts,
                    handle_action_story,
                ),
            );
    }
}
