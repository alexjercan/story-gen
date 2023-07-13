mod chatgpt;
mod fakeyou;
mod input;
mod parser;

use bevy::prelude::*;
pub struct StoryPlugin;

impl Plugin for StoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<StoryState>().add_plugins((
            input::StoryInputPlugin,
            chatgpt::ChatGPTPlugin,
            parser::ParserPlugin,
            fakeyou::FakeYouPlugin,
        ));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum StoryState {
    #[default]
    Idle,
    Simulation,
    Paused,
}
