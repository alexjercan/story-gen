mod components;
mod layout;
mod systems;

use bevy::prelude::*;

use systems::*;

use crate::AppState;

pub struct StoryPlugin;

impl Plugin for StoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<StoryState>()
            .add_systems(OnEnter(AppState::Story), spawn_hud_menu)
            .add_systems(OnExit(AppState::Story), despawn_hud_menu)
            // TODO: move this to a separate plugin
            // should I do it too for the above two systems?
            // Copilot: I think I should.
            // But maybe not, because it will be the full hud. I don't know yet
            // Maybe I will split the hud into components. That seems like a good idea
            .add_systems(Update, (interact_with_continue_button, show_hud_menu).run_if(in_state(StoryState::Idle)))
            .add_systems(Update, hide_hud_menu.run_if(not(in_state(StoryState::Idle))));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum StoryState {
    #[default]
    Idle,
    Simulation,
    Paused,
}
