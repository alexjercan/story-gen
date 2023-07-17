mod events;
mod resources;
mod systems;

use crate::AppState;
use bevy::prelude::*;
use chatgpt::*;
pub use events::{CreatedActionStoryEvent, InputPromptEvent};
use fakeyou::*;
use parser::*;
use resources::*;
use std::collections::HashMap;
use systems::*;

#[derive(Debug, Clone)]
pub enum ActionStory {
    Say(String, String, Option<Handle<AudioSource>>),
    Comment(String),
}

pub struct PipelinePlugin;

// TODO: this should be an asset
const SYSTEM: &str = r#"""You are given the formal language for describing a story. The description of
each instruction is as follows:

- `comment("description")` Insert a comment in the story.
- `say("character", "dialogue")` Make a character speak.

Do NOT use any other instruction than the ones listed above.

See the following script as an example of how your script MUST to be formatted:

```
comment("The story takes place inside Rick's garage. Rick is trying to fix the portal gun.")

say("Rick", "I need to fix this portal gun.")
say("Morty", "Oh jeez, Rick! There is an alien in the living room.")
say("Rick", "Frick, Morty! I have to fix the portal gun so I can get rid of the alien.")
```

You are in charge of creating a Rick and Morty story. Available
characters: Rick, Morty. Do NOT use extra characters. The
characters will use some light profanity like frick and crap. Make
sure to include catch phrases like "Oh jeez, Rick" for Morty and
"Wubba Lubba dub-dub" for Rick. Make sure to include nerdy and
sci-fi jokes."""#;

impl Plugin for PipelinePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActionsQueue>()
            .init_resource::<SayQueue>()
            .add_event::<InputPromptEvent>()
            .add_event::<CreatedActionStoryEvent>()
            .add_plugins((
                ChatGPTPlugin::default().with_system_prompt(SYSTEM),
                ParserPlugin,
                FakeYouPlugin::default().with_names(HashMap::from_iter(vec![
                    ("Rick".to_string(), "TM:ebgxj0j4fvzp".to_string()),
                    ("Morty".to_string(), "TM:mcvca56k5d5e".to_string()),
                ])),
            ))
            .add_systems(
                Update,
                (
                    handle_created_text,
                    handle_created_story,
                    handle_created_actions,
                    handle_created_tts,
                    handle_action_story,
                )
                    .run_if(in_state(AppState::Story)),
            );
    }
}
