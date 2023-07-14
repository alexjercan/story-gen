use bevy::tasks::AsyncComputeTaskPool;
use bevy::{prelude::*, tasks::Task};
use futures_lite::future;
use openai_api_rust::chat::*;
use openai_api_rust::*;
use std::fmt::Display;

// Expose the ChatBody struct from the openai_api_rust crate
pub use openai_api_rust::chat::ChatBody;

// The ChatGPTResult type is used to wrap the result of the ChatGPT call
pub type ChatGPTResult<T> = Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    ApiError(String),
    RequestError(String),
    StoryError(String),
}

impl From<openai_api_rust::Error> for Error {
    fn from(err: openai_api_rust::Error) -> Self {
        match err {
            openai_api_rust::Error::ApiError(err) => Error::ApiError(err),
            openai_api_rust::Error::RequestError(err) => Error::RequestError(err),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::ApiError(err) => write!(f, "API error: {}", err),
            Error::RequestError(err) => write!(f, "Request error: {}", err),
            Error::StoryError(err) => write!(f, "Story error: {}", err),
        }
    }
}

// This component is used to run the ChatGPT call on another thread
#[derive(Component)]
struct ChatGPTTask(Task<ChatGPTResult<String>>);

// The plugin will listen for this event to trigger the ChatGPT call
#[derive(Event, Debug)]
pub struct InputChatEvent(pub String);

// This event is triggered when a story is generated by ChatGPT
#[derive(Event, Debug)]
pub struct CreatedStoryEvent(pub ChatGPTResult<String>);

// This resource is used to store the ChatGPT request state
#[derive(Resource, Debug, Deref, DerefMut)]
struct StoryChatBody(ChatBody);

// TODO: this is a workaround to be able to clone the ChatBody struct
impl From<&ChatBody> for StoryChatBody {
    fn from(body: &ChatBody) -> Self {
        Self(ChatBody {
            model: body.model.clone(),
            max_tokens: body.max_tokens,
            temperature: body.temperature,
            top_p: body.top_p,
            n: body.n,
            stream: body.stream,
            stop: body.stop.clone(),
            presence_penalty: body.presence_penalty,
            frequency_penalty: body.frequency_penalty,
            logit_bias: body.logit_bias.clone(),
            user: body.user.clone(),
            messages: body.messages.clone(),
        })
    }
}

impl Clone for StoryChatBody {
    fn clone(&self) -> Self {
        Self(ChatBody {
            model: self.model.clone(),
            max_tokens: self.max_tokens,
            temperature: self.temperature,
            top_p: self.top_p,
            n: self.n,
            stream: self.stream,
            stop: self.stop.clone(),
            presence_penalty: self.presence_penalty,
            frequency_penalty: self.frequency_penalty,
            logit_bias: self.logit_bias.clone(),
            user: self.user.clone(),
            messages: self.messages.clone(),
        })
    }
}

impl StoryChatBody {
    fn add_user_message(&mut self, message: String) {
        self.messages.push(Message {
            role: Role::User,
            content: message,
        });
    }

    fn add_assistant_message(&mut self, message: String) {
        self.messages.push(Message {
            role: Role::Assistant,
            content: message,
        });
    }
}

// This resource holds the OpenAI API key
#[derive(Resource, Debug, Deref, DerefMut, Clone)]
struct StoryChatAuth(Auth);

impl Default for StoryChatAuth {
    fn default() -> Self {
        // TODO: will have to handle errors
        let auth = Auth::from_env().unwrap();

        Self(auth)
    }
}

pub struct ChatGPTPlugin {
    body: ChatBody,
}

impl Default for ChatGPTPlugin {
    fn default() -> Self {
        Self {
            body: ChatBody {
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
                messages: Vec::default(),
            },
        }
    }
}

impl ChatGPTPlugin {
    pub fn new(body: ChatBody) -> Self {
        Self { body }
    }

    pub fn with_system_prompt(mut self, system_prompt: &str) -> Self {
        self.body.messages.push(Message {
            role: Role::System,
            content: system_prompt.to_string(),
        });
        self
    }
}

impl Plugin for ChatGPTPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputChatEvent>()
            .add_event::<CreatedStoryEvent>()
            .init_resource::<StoryChatAuth>()
            .insert_resource(StoryChatBody::from(&self.body))
            .add_systems(Update, (handle_input_chat, poll_chatgpt_task));
    }
}

fn handle_input_chat(
    mut commands: Commands,
    mut ev_input_chat: EventReader<InputChatEvent>,
    mut chat_body: ResMut<StoryChatBody>,
    auth: Res<StoryChatAuth>,
) {
    ev_input_chat.iter().for_each(|ev| {
        let message = ev.0.clone();
        chat_body.add_user_message(message);

        let auth = auth.clone();
        let chat_body = chat_body.clone();

        let thread_pool = AsyncComputeTaskPool::get();
        let task = thread_pool.spawn(async move {
            let openai = OpenAI::new(auth.0, "https://api.openai.com/v1/");

            openai
                .chat_completion_create(&chat_body)?
                .choices
                .get(0)
                .ok_or_else(|| Error::StoryError("No choice in response".to_string()))?
                .message
                .as_ref()
                .map(|m| m.content.clone())
                .ok_or_else(|| Error::StoryError("No message in response".to_string()))
        });

        commands.spawn(ChatGPTTask(task));
    });
}

fn poll_chatgpt_task(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ChatGPTTask)>,
    mut chat_body: ResMut<StoryChatBody>,
    mut ev_created_story: EventWriter<CreatedStoryEvent>,
) {
    let Some((entity, mut task)) = tasks.iter_mut().next() else { return };

    if let Some(action) = future::block_on(future::poll_once(&mut task.0)) {
        ev_created_story.send(CreatedStoryEvent(action.clone()));

        if let Ok(message) = action {
            chat_body.add_assistant_message(message)
        }

        commands.entity(entity).despawn();
    }
}
