use crate::input::CreatedTextEvent;
use crate::pipeline::InputPromptEvent;
use crate::SYSTEM_TEXT;
use crate::{components::MainCamera, selection_menu::SelectedStory};
use bevy::{prelude::*, window::PrimaryWindow};
use chatgpt::InputSystemEvent;
use fakeyou::{InputOptionsEvent, TTSOptions};

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

pub fn handle_started(
    mut ev_input_system: EventWriter<InputSystemEvent>,
    mut ev_input_options: EventWriter<InputOptionsEvent>,
    story: Res<SelectedStory>,
) {
    let story = story.0.as_ref().expect("unreachable - no story selected");

    let system = SYSTEM_TEXT.to_string() + (&story.system);
    ev_input_system.send(InputSystemEvent(system.clone()));
    ev_input_options.send(InputOptionsEvent(TTSOptions {
        voices: story.voices.clone(),
    }));
}
