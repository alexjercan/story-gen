use openai_api_rust::chat::*;
use openai_api_rust::*;

use crate::{Error, PipelineResult};

#[derive(Debug, Default)]
pub(crate) struct ChatGPT {
    messages: Vec<Message>,
}

impl ChatGPT {
    pub(crate) fn new(system: &str) -> Self {
        Self {
            messages: vec![Message {
                role: Role::System,
                content: system.to_string(),
            }],
        }
    }

    pub(crate) fn clear(&mut self) {
        self.messages.truncate(1);
    }

    pub(crate) fn generate(&mut self, prompt: &str) -> PipelineResult<String> {
        let auth = Auth::from_env().map_err(|e| Error::StoryError(e))?;
        let openai = OpenAI::new(auth, "https://api.openai.com/v1/");

        self.messages.push(Message {
            role: Role::User,
            content: prompt.to_string(),
        });

        let body = ChatBody {
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
            messages: self.messages.clone(),
        };

        let message = openai
            .chat_completion_create(&body)
            .map_err(|e| Error::StoryError(e.to_string()))?
            .choices[0]
            .message
            .as_ref()
            .map(|m| m.content.clone())
            .ok_or_else(|| Error::StoryError("No text found".to_string()))?;

        self.messages.push(Message {
            role: Role::Assistant,
            content: message.clone(),
        });

        Ok(message)
    }
}
