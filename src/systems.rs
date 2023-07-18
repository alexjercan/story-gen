use crate::components::MainCamera;
use crate::input::CreatedTextEvent;
use crate::loader::StoryAsset;
use crate::pipeline::InputPromptEvent;
use crate::resources::Stories;
use bevy::{prelude::*, window::PrimaryWindow};
use chatgpt::InputSystemEvent;
use fakeyou::{InputOptionsEvent, TTSOptions};

pub fn setup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut stories: ResMut<Stories>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        MainCamera,
    ));

    if let Ok(handles) = asset_server.load_folder("story") {
        stories.stories = handles
            .into_iter()
            .map(|h| h.typed::<StoryAsset>())
            .collect();
    }
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
    story_assets: Res<Assets<StoryAsset>>,
    asset_server: Res<AssetServer>,
) {
    // TODO: this is hardcoded for now. Make a menu where you can select from
    // the available stories.
    let asset_handle: Handle<StoryAsset> = asset_server.load("story/rick_and_morty.story.ron");
    let story = story_assets.get(&asset_handle).unwrap();

    ev_input_system.send(InputSystemEvent(story.system.clone()));
    ev_input_options.send(InputOptionsEvent(TTSOptions {
        voices: story.voices.clone(),
    }));
}
