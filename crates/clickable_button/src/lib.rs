mod components;
mod systems;

use bevy::prelude::*;
pub use components::ClickableButton;
use systems::*;

pub struct ClickableButtonPlugin;

impl Plugin for ClickableButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, interact_with_button);
    }
}
