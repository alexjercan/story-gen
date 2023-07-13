use bevy::prelude::*;
use openai_api_rust::chat::*;
use openai_api_rust::*;

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

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct StoryChatBody {
    pub body: ChatBody,
}

impl Default for StoryChatBody {
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
                messages: vec![Message {
                    role: Role::System,
                    content: SYSTEM.to_string(),
                }],
            },
        }
    }
}

impl StoryChatBody {
    pub fn add_user_message(&mut self, message: String) {
        self.body.messages.push(Message {
            role: Role::User,
            content: message,
        });
    }

    pub fn add_assistant_message(&mut self, message: String) {
        self.body.messages.push(Message {
            role: Role::Assistant,
            content: message,
        });
    }
}

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct StoryChatAuth(pub Auth);

impl Default for StoryChatAuth {
    fn default() -> Self {
        // TODO: will have to handle errors
        let auth = Auth::from_env().unwrap();

        Self(auth)
    }
}
