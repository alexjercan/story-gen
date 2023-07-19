use super::events::*;
use super::resources::*;
use super::ActionStory;
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_sysfail::*;
use chatgpt::*;
use fakeyou::*;
use log::{info, warn};

pub fn handle_created_text(
    mut ev_input_prompt: EventReader<InputPromptEvent>,
    mut ev_input_chat: EventWriter<InputChatEvent>,
) {
    ev_input_prompt.iter().for_each(|ev| {
        info!("handle_created_text text={:?}", ev.0);

        let text = format!("[{{\"comment\": {{\"text\": \"{}\"}}}}]", ev.0);

        ev_input_chat.send(InputChatEvent(text));
    });
}

#[sysfail(log)]
pub fn handle_created_story(
    mut ev_created_story: EventReader<CreatedStoryEvent>,
    mut ev_input_say: EventWriter<InputSayEvent>,
    mut actions_queue: ResMut<ActionsQueue>,
) -> Result<()> {
    let actions = ev_created_story
        .iter()
        .fold(Ok(vec![]), |acc: Result<_>, ev| {
            let story = ev.0.as_ref().map_err(|err| anyhow::anyhow!("{}", err))?;

            let actions = serde_json::from_str::<Vec<Action>>(story)?;

            Ok(acc?.into_iter().chain(actions.into_iter()).collect())
        })?;

    if actions.is_empty() {
        return Ok(());
    }

    actions
        .iter()
        .chain(std::iter::once(&Action::EndOfStory))
        .for_each(|action| {
            actions_queue.actions.push_back(action.clone());

            info!("handle_created_story action={:?}", action);

            match action {
                Action::Say { name, text } => {
                    ev_input_say.send(InputSayEvent {
                        name: name.clone(),
                        text: text.clone(),
                    });
                }
                Action::Comment { .. } => {}
                Action::EndOfStory => {}
            };
        });

    Ok(())
}

pub fn handle_created_tts(
    mut ev_created_tts: EventReader<CreatedTTSEvent>,
    mut say_queue: ResMut<SayQueue>,
) {
    ev_created_tts.iter().for_each(|ev| {
        info!("handle_created_tts tts={:?}", ev.0);

        let tts = match &ev.0 {
            Ok(tts) => Some(tts.clone()),
            Err(err) => {
                warn!("TTS error: {}", err);
                None
            }
        };

        say_queue.say.push_back(tts);
    });
}

pub fn handle_action_story(
    mut actions_queue: ResMut<ActionsQueue>,
    mut say_queue: ResMut<SayQueue>,
    mut ev_created_action_story: EventWriter<CreatedActionStoryEvent>,
) {
    if let Some(action) = actions_queue.actions.front() {
        match action {
            Action::Say { name, text } => {
                let has_say = say_queue.say.front().is_some();

                if has_say {
                    let say = say_queue.say.pop_front().unwrap();

                    info!("{}({:?}): {}", name, say.is_some(), text);

                    ev_created_action_story.send(CreatedActionStoryEvent(ActionStory::Say {
                        name: name.clone(),
                        text: text.clone(),
                        tts: say,
                    }));

                    actions_queue.actions.pop_front();
                }
            }
            Action::Comment { text } => {
                info!("// {}", text);

                ev_created_action_story.send(CreatedActionStoryEvent(ActionStory::Comment {
                    text: text.clone(),
                }));

                actions_queue.actions.pop_front();
            }
            Action::EndOfStory => {
                info!("End of story");

                ev_created_action_story.send(CreatedActionStoryEvent(ActionStory::EndOfStory));

                actions_queue.actions.pop_front();
            }
        }
    }
}
