use super::events::InputStoryEvent;
use super::resources::StoryChatAuth;
use super::{components::StoryGPTLoader, resources::StoryChatBody};
use crate::story::input::InputTextEvent;
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use futures_lite::future;
use openai_api_rust::chat::*;
use openai_api_rust::*;

pub fn handle_input_text(
    mut commands: Commands,
    mut ev_input_text: EventReader<InputTextEvent>,
    mut chat_body: ResMut<StoryChatBody>,
    auth: Res<StoryChatAuth>,
) {
    let Some(message) = ev_input_text.iter().next().map(|ev| ev.0.clone()) else { return };
    chat_body.add_user_message(message);

    let auth = auth.clone();
    let chat_body = chat_body.clone();

    let thread_pool = AsyncComputeTaskPool::get();
    let task = thread_pool.spawn(async move {
        let openai = OpenAI::new(auth.0, "https://api.openai.com/v1/");

        openai.chat_completion_create(&chat_body)?.choices[0]
            .message
            .as_ref()
            .map(|m| m.content.clone())
            .ok_or_else(|| super::error::Error::StoryError("No message in response".to_string()))
    });

    commands.spawn(StoryGPTLoader(task));
}

pub fn poll_story_loader_task(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut StoryGPTLoader)>,
    mut chat_body: ResMut<StoryChatBody>,
    mut ev_input_story: EventWriter<InputStoryEvent>,
) {
    let Some((entity, mut task)) = tasks.iter_mut().next() else { return };

    if let Some(action) = future::block_on(future::poll_once(&mut task.0)) {
        match action {
            Ok(message) => {
                ev_input_story.send(InputStoryEvent(message.clone()));
                chat_body.add_assistant_message(message)
            }
            Err(s) => {
                // TODO: Handle error
                panic!("Error: {:?}", s);
            }
        }

        commands.entity(entity).despawn();
    }
}
