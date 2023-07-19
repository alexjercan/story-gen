mod components;
mod events;
mod layout;
mod resources;
mod systems;

use bevy::prelude::*;
pub use events::{CreatedEndOfStoryEvent, InputActionStoryEvent};
use resources::*;
use systems::*;

use crate::AppState;

pub struct StoryPlugin;

impl Plugin for StoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<StoryState>()
            .init_resource::<StoryActions>()
            .add_event::<InputActionStoryEvent>()
            .add_event::<CreatedEndOfStoryEvent>()
            .add_systems(OnEnter(AppState::Story), spawn_subtitle_hud)
            .add_systems(
                OnExit(AppState::Story),
                (despawn_subtitle_hud, despawn_story_actions),
            )
            .add_systems(
                Update,
                (
                    handle_created_action, handle_quit_to_menu,
                    (spawn_story_actions, subtitle_clean_system)
                        .run_if(in_state(StoryState::Spawn)),
                    (check_story_action, subtitle_system, audio_system)
                        .run_if(in_state(StoryState::Playing)),
                    end_of_story.run_if(in_state(StoryState::EndOfStory)),
                )
                    .run_if(in_state(AppState::Story)),
            );
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum StoryState {
    #[default]
    Spawn,
    Playing,
    EndOfStory,
}
