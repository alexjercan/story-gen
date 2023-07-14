use super::components::*;
use super::resources::*;
use super::InputActionStoryEvent;
use super::{CommentAction, SayAction, StoryAction};
use crate::story::parser::InputActionEvent;
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use fakeyou_api::tts::*;
use fakeyou_api::util::tts::*;
use fakeyou_api::*;
use futures_lite::future;
use story_gen_parser::Action;

fn make_tts_request(options: &TTSOptions, name: &str, text: &str) -> Result<Vec<u8>, String> {
    let fakeyou = FakeYou::default();

    let name = options
        .names
        .get(name)
        .ok_or_else(|| format!("Name {} not found in names list", name))?;

    let inference_body = InferenceBody::new(name, text);

    fakeyou
        .create_tts_task(&inference_body)
        .map(|t| t.bytes)
        .map_err(|e| e.to_string())
}

pub fn handle_input_action(
    mut commands: Commands,
    mut ev_input_action: EventReader<InputActionEvent>,
    options: Res<TTSOptions>,
) {
    ev_input_action.into_iter().for_each(|ev| match &ev.0 {
        Action::Comment(comment) => {
            let thread_pool = AsyncComputeTaskPool::get();

            let comment = comment.clone();
            let task = thread_pool.spawn(async move {
                StoryAction::Comment(CommentAction {
                    text: comment.clone(),
                })
            });

            commands.spawn(InterpreterActionLoader(task));
        }
        Action::Say(name, text) => {
            let thread_pool = AsyncComputeTaskPool::get();

            let name = name.clone();
            let text = text.clone();
            let options = options.clone();
            let task = thread_pool.spawn(async move {
                let bytes = make_tts_request(&options, &name, &text);

                StoryAction::Say(SayAction {
                    name: name.clone(),
                    text: text.clone(),
                    tts: bytes,
                })
            });

            commands.spawn(InterpreterActionLoader(task));
        }
    });
}

pub fn poll_action_loader_task(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut InterpreterActionLoader)>,
    mut actions: ResMut<StoryActions>,
    mut ev_input_story: EventWriter<InputActionStoryEvent>,
) {
    let Some((entity, mut task)) = tasks.iter_mut().next() else { return };

    if let Some(action) = future::block_on(future::poll_once(&mut task.0)) {
        match &action {
            StoryAction::Say(SayAction { name, text, tts }) => {
                println!("{}: {}", name, text);
                match tts {
                    Ok(bytes) => std::fs::write(format!("{text}.wav"), bytes).unwrap(),
                    Err(text) => println!("Audio Error: {}", text),
                }
            }
            StoryAction::Comment(CommentAction { text }) => {
                println!("// {}", text);
            }
        }

        actions.push(action.clone());

        ev_input_story.send(InputActionStoryEvent(action));

        commands.entity(entity).despawn();
    }
}
