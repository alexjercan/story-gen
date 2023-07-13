mod components;
mod debug;
mod main_menu;
mod options;
mod story;
mod styles;
mod systems;

use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_systems(Startup, setup)
        .add_plugins((
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
