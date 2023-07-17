mod components;
mod debug;
mod input;
mod main_menu;
mod options;
mod pipeline;
mod styles;
mod systems;

use bevy::prelude::*;
use clickable_button::ClickableButtonPlugin;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_systems(Startup, setup)
        .add_plugins((
            ClickableButtonPlugin,
            main_menu::MainMenuPlugin,
            options::OptionsMenuPlugin,
            input::StoryInputPlugin,
            pipeline::PipelinePlugin,
        ))
        // .add_plugins(debug::DebugPlugin)
        .add_systems(
            Update,
            handle_created_text.run_if(in_state(AppState::Story)),
        )
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    OptionsMenu,
    Story,
}
