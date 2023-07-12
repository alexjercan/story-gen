use bevy::app::AppExit;
use bevy::prelude::*;

use super::components::*;
use super::layout::*;

use crate::resources::StoryGenAuth;
use crate::styles;
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
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    auth: Res<StoryGenAuth>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = styles::color::FOCUS.into();
                // TODO: Show a popup message and don't change state if there's no auth
                // the user will be able to set it up in the options menu or environment variables
                match auth.auth {
                    Some(_) => app_state_next_state.set(AppState::Story),
                    None => println!("No auth, please set OPENAI_API_KEY..."),
                };
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

pub fn interact_with_options_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<OptionsButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = styles::color::FOCUS.into();
                app_state_next_state.set(AppState::OptionsMenu);
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

pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = styles::color::FOCUS.into();
                app_exit_event_writer.send(AppExit);
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
