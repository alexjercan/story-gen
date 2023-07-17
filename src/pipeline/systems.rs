use super::events::*;
use super::resources::*;
use super::ActionStory;
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_sysfail::*;
use chatgpt::*;
use fakeyou::*;
use parser::*;

pub fn handle_created_text(
    mut ev_input_prompt: EventReader<InputPromptEvent>,
    mut ev_input_chat: EventWriter<InputChatEvent>,
) {
    ev_input_prompt.iter().for_each(|ev| {
        ev_input_chat.send(InputChatEvent(ev.0.clone()));
    });
}

#[sysfail(log)]
pub fn handle_created_story(
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
pub fn handle_created_actions(
    mut ev_created_actions: EventReader<CreatedActionsEvent>,
    mut ev_input_say: EventWriter<InputSayEvent>,
    mut actions_queue: ResMut<ActionsQueue>,
) -> Result<()> {
    ev_created_actions
        .iter()
        .map(|ev| {
            let actions = ev.0.clone().map_err(|e| anyhow::anyhow!(e))?;

            Ok(actions.iter().for_each(|action| {
                actions_queue.actions.push_back(action.clone());

                match action {
                    Action::Say(name, text) => {
                        ev_input_say.send(InputSayEvent(SayAction {
                            name: name.clone(),
                            text: text.clone(),
                        }));
                    }
                    Action::Comment(_) => {}
                };
            }))
        })
        .fold(Ok(()), |acc, res| acc.and(res))
}

pub fn handle_created_tts(
    mut ev_created_tts: EventReader<CreatedTTSEvent>,
    mut say_queue: ResMut<SayQueue>,
) {
    ev_created_tts.iter().for_each(|ev| {
        let tts = match ev.0.clone() {
            Ok(tts) => Some(tts),
            Err(err) => {
                println!("TTS error: {}", err);
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
            Action::Say(name, text) => {
                let has_say = say_queue.say.front().is_some();

                if has_say {
                    let say = say_queue.say.pop_front().unwrap();

                    println!("{}({:?}): {}", name, say.is_some(), text);

                    ev_created_action_story.send(CreatedActionStoryEvent(ActionStory::Say(
                        name.clone(),
                        text.clone(),
                        say,
                    )));

                    actions_queue.actions.pop_front();
                }
            }
            Action::Comment(text) => {
                println!("// {}", text);

                ev_created_action_story
                    .send(CreatedActionStoryEvent(ActionStory::Comment(text.clone())));

                actions_queue.actions.pop_front();
            }
        }
    }
}
