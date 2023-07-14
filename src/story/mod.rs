mod input;

use std::collections::HashMap;

use crate::AppState;
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_sysfail::*;
use chatgpt::*;
use fakeyou::*;
use input::*;
use parser::*;

pub struct StoryPlugin;

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

impl Plugin for StoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<StoryState>()
            .add_plugins((
                StoryInputPlugin,
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
                )
                    .run_if(in_state(AppState::Story)),
            );
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum StoryState {
    #[default]
    Idle,
    Simulation,
    Paused,
}

fn handle_created_text(
    mut ev_created_text: EventReader<CreatedTextEvent>,
    mut ev_input_chat: EventWriter<InputChatEvent>,
) {
    ev_created_text.iter().for_each(|ev| {
        ev_input_chat.send(InputChatEvent(ev.0.clone()));
    });
}

#[sysfail(log)]
fn handle_created_story(
    mut ev_created_story: EventReader<CreatedStoryEvent>,
    mut ev_input_text: EventWriter<InputTextEvent>,
) -> Result<()> {
    ev_created_story
        .iter()
        .map(|ev| {
            let story = ev.0.clone().map_err(|e| anyhow::anyhow!(e))?;

            Ok(ev_input_text.send(InputTextEvent(story)))
        })
        .fold(Ok(()), |acc, res| acc.and(res))
}

#[sysfail(log)]
fn handle_created_actions(
    mut ev_created_actions: EventReader<CreatedActionsEvent>,
    mut ev_input_say: EventWriter<InputSayEvent>,
) -> Result<()> {
    ev_created_actions
        .iter()
        .map(|ev| {
            let actions = ev.0.clone().map_err(|e| anyhow::anyhow!(e))?;

            Ok(actions.iter().for_each(|action| match action {
                Action::Say(name, text) => {
                    println!("{}: {}", name, text);
                    ev_input_say.send(InputSayEvent(SayAction {
                        name: name.clone(),
                        text: text.clone(),
                    }));
                }
                Action::Comment(description) => {
                    println!("// {}", description);
                }
            }))
        })
        .fold(Ok(()), |acc, res| acc.and(res))
}

#[sysfail(log)]
fn handle_created_tts(mut ev_created_tts: EventReader<CreatedTTSEvent>) -> Result<()> {
    ev_created_tts
        .iter()
        .map(|ev| {
            let tts = ev.0.clone().map_err(|e| anyhow::anyhow!(e))?;
            // TODO: link tts to the action

            Ok(())
        })
        .fold(Ok(()), |acc, res| acc.and(res))
}
