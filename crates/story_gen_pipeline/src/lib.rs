mod chatgpt;
mod fakeyou;
mod parse;

pub mod error;
pub use error::*;

use story_gen_parser::Action;

pub enum StoryAction {
    Comment(String),
    Say(String, String, Vec<u8>),
}

pub fn pipeline(prompt: &str) -> PipelineResult<Vec<StoryAction>> {
    let story = chatgpt::generate_story(&prompt)?;
    let actions = parse::parse_story(&story)?;

    actions.into_iter().map(|a| {
        println!("{:?}", a);

        let sa = match a {
            Action::Comment(text) => StoryAction::Comment(text),
            Action::Say(name, text) => {
                let audio = fakeyou::generate_audio(&name, &text)?;
                StoryAction::Say(name, text, audio)
            }
        };
        Ok(sa)
    }).collect::<PipelineResult<Vec<_>>>()
}
