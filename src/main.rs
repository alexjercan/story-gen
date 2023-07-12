mod main_menu;
mod options;
mod systems;
mod styles;
mod resources;

use systems::*;
use resources::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .init_resource::<StoryGenAuth>()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_systems(Startup, spawn_camera)
        .add_plugins((main_menu::MainMenuPlugin, options::OptionsMenuPlugin))
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    OptionsMenu,
    Story,
}
