mod components;
mod debug;
mod main_menu;
mod options;
mod story;
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
            story::StoryPlugin,
        ))
        // .add_plugins(debug::DebugPlugin)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    OptionsMenu,
    Story,
}
