use super::resources::StoryChatAuth;
use super::{components::StoryGPTLoader, error::StoryGPTResult, resources::StoryChatBody};
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
    let message = ev_input_text
        .iter()
        .fold(String::default(), |acc, ev| acc + &ev.0);
    chat_body.add_user_message(message);

    // TODO: how can overcome this inconvenience?
    let chat_body = ChatBody {
        model: "gpt-3.5-turbo".to_string(),
        max_tokens: None,
        temperature: None,
        top_p: None,
        n: None,
        stream: None,
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        messages: chat_body.body.messages.clone(),
    };

    let auth = auth.0.clone();
    let thread_pool = AsyncComputeTaskPool::get();

    let task = thread_pool.spawn(async move {
        let openai = OpenAI::new(auth, "https://api.openai.com/v1/");

        let message = openai
            .chat_completion_create(&chat_body)
            .unwrap() // TODO: handle error
            .choices[0]
            .message
            .as_ref()
            .map(|m| m.content.clone())
            .unwrap(); // TODO: handle error

        StoryGPTResult::Ok(message)
    });

    commands.spawn(StoryGPTLoader(task));
}

pub fn poll_story_loader_task(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut StoryGPTLoader)>,
) {
    let (entity, mut task) = tasks.get_single_mut().unwrap();

    if let Some(action) = future::block_on(future::poll_once(&mut task.0)) {
        match action {
            Ok(s) => {
                println!("Success: {}", s);
            }
            Err(s) => {
                println!("Error: {:?}", s);
            }
        }

        commands.entity(entity).despawn();
    }
}
