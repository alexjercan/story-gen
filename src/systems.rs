use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::MainCamera;
use crate::input::CreatedTextEvent;
use crate::pipeline::InputPromptEvent;

pub fn setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        MainCamera,
    ));
}

pub fn handle_created_text(
    mut ev_created_text: EventReader<CreatedTextEvent>,
    mut ev_input_chat: EventWriter<InputPromptEvent>,
) {
    ev_created_text.iter().for_each(|ev| {
        ev_input_chat.send(InputPromptEvent(ev.0.clone()));
    });
}
