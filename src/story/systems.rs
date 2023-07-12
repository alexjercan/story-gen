use bevy::prelude::*;

use super::components::*;
use super::layout::*;

use crate::styles;
use crate::AppState;

pub fn spawn_hud_menu(mut commands: Commands) {
    build_hud_menu(&mut commands);
}

pub fn despawn_hud_menu(mut commands: Commands, hud_menu_query: Query<Entity, With<HudMenu>>) {
    if let Ok(hud_menu_entity) = hud_menu_query.get_single() {
        commands.entity(hud_menu_entity).despawn_recursive();
    }
}

pub fn show_hud_menu(mut hud_menu_query: Query<&mut Visibility, With<HudMenu>>) {
    if let Ok(mut hud_menu_visibility) = hud_menu_query.get_single_mut() {
        *hud_menu_visibility = Visibility::Visible;
    }
}

pub fn hide_hud_menu(mut hud_menu_query: Query<&mut Visibility, With<HudMenu>>) {
    if let Ok(mut hud_menu_visibility) = hud_menu_query.get_single_mut() {
        *hud_menu_visibility = Visibility::Hidden;
    }
}

pub fn interact_with_continue_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ContinueButton>),
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
                *background_color = styles::color::PRIMARY.into();
            }
        }
    }
}
