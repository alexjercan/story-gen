// TODO: Still need to separate the pipeline into a pipeline for each component because I have to
// handle if something breaks better. Right now if something breaks the other components fail too.
// But for example if the audio fails for on step then the other steps should still work.
mod chatgpt;
mod fakeyou;
mod parse;

pub mod error;
use std::collections::{VecDeque, HashMap};

pub use error::*;

use story_gen_parser::Action;

#[derive(Debug, Clone)]
pub enum StoryAction {
    Comment(String),
    Say(String, String, Vec<u8>),
}

#[derive(Debug, Default)]
pub struct Pipeline {
    prompts: VecDeque<String>,
    actions: VecDeque<Action>,

    chatgpt: chatgpt::ChatGPT,
    parser: parse::Parser,
    fakeyou: fakeyou::FakeYouTTS,
}

impl Pipeline {
    pub fn new(system: &str, name_to_id: HashMap<String, String>) -> Self {
        let chatgpt = chatgpt::ChatGPT::new(system);
        let parser = parse::Parser::new();
        let fakeyou_tts = fakeyou::FakeYouTTS::new(name_to_id);

        Self {
            prompts: VecDeque::new(),
            actions: VecDeque::new(),
            chatgpt,
            parser,
            fakeyou: fakeyou_tts,
        }
    }

    pub fn push_back(&mut self, prompt: &str) {
        self.prompts.push_back(prompt.to_string());
    }

    pub fn is_empty(&self) -> bool {
        self.prompts.is_empty() && self.actions.is_empty()
    }

    pub fn clear(&mut self) {
        self.prompts.clear();
        self.actions.clear();
        self.chatgpt.clear();
        self.parser.clear();
    }
}

impl Iterator for Pipeline {
    type Item = PipelineResult<StoryAction>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(action) = self.actions.pop_front() {
            Some(match action {
                Action::Comment(text) => Ok(StoryAction::Comment(text)),
                Action::Say(name, text) => match self.fakeyou.generate(&name, &text) {
                    Ok(audio) => Ok(StoryAction::Say(name, text, audio)),
                    Err(e) => Err(e),
                },
            })
        } else if let Some(prompt) = self.prompts.pop_front() {
            match self.chatgpt.generate(&prompt) {
                Ok(story) => match self.parser.parse(&story) {
                    Ok(actions) => {
                        self.actions = actions.into_iter().collect();
                        self.next()
                    }
                    Err(e) => Some(Err(e)),
                },
                Err(e) => Some(Err(e)),
            }
        } else {
            None
        }
    }
}
