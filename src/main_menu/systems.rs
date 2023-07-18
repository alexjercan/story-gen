use bevy::app::AppExit;
use bevy::prelude::*;

use super::components::*;
use super::layout::*;

use crate::AppState;

pub fn spawn_main_menu(mut commands: Commands) {
    build_main_menu(&mut commands);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn interact_with_play_button(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok(Interaction::Pressed) = button_query.get_single_mut() {
        app_state_next_state.set(AppState::SelectionMenu)
    }
}

pub fn interact_with_options_button(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<OptionsButton>)>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok(Interaction::Pressed) = button_query.get_single_mut() {
        app_state_next_state.set(AppState::OptionsMenu);
    }
}

pub fn interact_with_quit_button(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok(Interaction::Pressed) = button_query.get_single_mut() {
        app_exit_event_writer.send(AppExit);
    }
}
