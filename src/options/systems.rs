use bevy::prelude::*;

use super::components::*;
use super::layout::*;

use crate::styles;
use crate::AppState;

pub fn spawn_options_menu(mut commands: Commands) {
    build_main_menu(&mut commands);
}

pub fn despawn_options_menu(mut commands: Commands, options_menu_query: Query<Entity, With<OptionsMenu>>) {
    if let Ok(options_menu_entity) = options_menu_query.get_single() {
        commands.entity(options_menu_entity).despawn_recursive();
    }
}

pub fn interact_with_back_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BackButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = styles::color::FOCUS.into();
                app_state_next_state.set(AppState::MainMenu);
            }
            Interaction::Hovered => {
                *background_color = styles::color::HOVER.into();
            }
            Interaction::None => {
                *background_color = styles::color::SECONDARY.into();
            }
        }
    }
}
