use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
    text::BreakLineOn,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_mod_sysfail::*;

use futures_lite::future;
use story_gen_pipeline::*;

const PROMPT: &str = r#"""You are given the formal language for describing a story. The description of
each instruction is as follows:

- `comment("description")` Insert a comment in the story.
- `say("character", "dialogue")` Make a character speak.

Do NOT use any other instruction than the ones listed above.

See the following script as an example of how your script MUST to be formatted:

```
comment("The story takes place inside Rick's garage. Rick is trying to fix the portal gun.")

say("Rick", "I need to fix this portal gun.")
say("Morty", "Oh jeez, Rick! There is an alien in the living room.")
say("Rick", "Frick, Morty! I have to fix the portal gun so I can get rid of the alien.")
```

You are in charge of creating a Rick and Morty story. Available
characters: Rick, Morty. Do NOT use extra characters. The
characters will use some light profanity like frick and crap. Make
sure to include catch phrases like "Oh jeez, Rick" for Morty and
"Wubba Lubba dub-dub" for Rick. Make sure to include nerdy and
sci-fi jokes."""#;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Default, Resource)]
pub struct UiState {
    pub input_box: String,
}

#[derive(Default, Resource, Deref, DerefMut)]
pub struct Story(pub String);

pub fn input_box_ui(
    mut state: ResMut<UiState>,
    mut story: ResMut<Story>,
    mut contexts: EguiContexts,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    let mut submit = false;
    let mut replay = false;

    egui::TopBottomPanel::bottom("input_box").show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered_justified(|ui| {
            ui.code_editor(&mut state.input_box);
            submit = ui.button("Submit").clicked();
            if !story.0.is_empty() {
                replay = ui.button("Replay").clicked();
            }
        });
    });

    if submit {
        contexts.ctx_mut().request_repaint();
        story.0 = format!("{}\n\n{}", PROMPT, state.input_box);
        state.input_box.clear();
        app_state_next_state.set(AppState::Loading);
    }

    if replay {
        contexts.ctx_mut().request_repaint();
        app_state_next_state.set(AppState::Story);
    }
}

#[derive(Component)]
pub struct StoryActionLoader(pub Task<PipelineResult<Vec<StoryAction>>>);

#[derive(Default, Resource, Deref, DerefMut)]
pub struct StoryActions(pub Vec<StoryAction>);

pub fn spawn_story_loader(mut commands: Commands, story: Res<Story>) {
    let thread_pool = AsyncComputeTaskPool::get();
    let story = story.0.clone();
    // TODO: Should try to optimize the pipeline
    // Right now the pipeline loads the entire story in the background
    // But it would maybe be better to load in chunks while the story plays
    let task = thread_pool.spawn(async move { pipeline(&story) });

    commands.spawn(StoryActionLoader(task));
}

pub fn check_story_loader(
    mut tasks: Query<&mut StoryActionLoader>,
    mut story_actions: ResMut<StoryActions>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    let mut task = tasks.get_single_mut().unwrap();

    if let Some(story) = future::block_on(future::poll_once(&mut task.0)) {
        match story {
            Ok(story) => {
                story_actions.0 = story;
                app_state_next_state.set(AppState::Story);
            }
            Err(err) => {
                println!("Error: {}", err.to_string());
                app_state_next_state.set(AppState::Input);
            }
        }
    }
}

pub fn despawn_story_loader(mut commands: Commands, tasks: Query<Entity, With<StoryActionLoader>>) {
    for entity in tasks.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Component, Debug)]
pub struct StoryActionValue;

#[derive(Component, Debug)]
pub struct StoryActionNode(pub Option<Entity>);

#[derive(Component, Debug)]
pub struct StoryActionTimer(pub Timer);

#[derive(Component, Debug)]
pub struct StoryActionSubtitle(pub String);

#[derive(Component, Debug)]
pub struct StoryActionAudio;

#[derive(Default, Resource, Debug)]
pub struct CurrentAction(pub Option<Entity>);

pub fn spawn_story_actions(
    mut commands: Commands,
    actions: Res<StoryActions>,
    mut assets: ResMut<Assets<AudioSource>>,
) {
    let current_action = actions.iter().rev().fold(None, |next, action| {
        let entity = match action {
            StoryAction::Comment(text) => commands.spawn((
                StoryActionValue,
                StoryActionNode(next),
                StoryActionTimer(Timer::from_seconds(3.0, TimerMode::Once)),
                StoryActionSubtitle(format!("{}", text)),
            )),
            StoryAction::Say(name, text, audio) => {
                let audio_source = AudioSource {
                    bytes: audio.as_slice().into(),
                };
                let handle = assets.add(audio_source);

                // TODO: How can I find the duration of the audio?
                let duration = audio.len() as f32 / 44100.0;

                commands.spawn((
                    StoryActionValue,
                    StoryActionNode(next),
                    StoryActionTimer(Timer::from_seconds(duration, TimerMode::Once)),
                    StoryActionSubtitle(format!("{}: {}", name, text)),
                    StoryActionAudio,
                    AudioBundle {
                        source: handle,
                        settings: PlaybackSettings::ONCE.paused(),
                    },
                ))
            }
        }
        .id();

        Some(entity)
    });

    commands.insert_resource(CurrentAction(current_action));
}

pub fn despawn_actions(mut commands: Commands, actions: Query<Entity, With<StoryActionValue>>) {
    for entity in actions.iter() {
        commands.entity(entity).despawn();
    }
}

#[quick_sysfail]
pub fn next_action_system(
    time: Res<Time>,
    mut current_action: ResMut<CurrentAction>,
    mut actions: Query<(&StoryActionNode, &mut StoryActionTimer)>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    match current_action.0 {
        Some(entity) => {
            let (node, mut timer) = actions.get_mut(entity).ok()?;

            if timer.0.tick(time.delta()).just_finished() {
                current_action.0 = node.0;
            }
        }
        None => {
            app_state_next_state.set(AppState::Input);
        }
    }
}

#[quick_sysfail]
pub fn subtitle_system(
    mut text_query: Query<&mut Text, With<SubtitleTextHud>>,
    current_action: Res<CurrentAction>,
    actions: Query<&StoryActionSubtitle>,
) {
    let entity = current_action.0?;
    let value = actions.get(entity).ok()?;

    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("{}", value.0);
    }
}

#[quick_sysfail]
pub fn audio_system(
    current_action: Res<CurrentAction>,
    actions: Query<&AudioSink, With<StoryActionAudio>>,
) {
    let entity = current_action.0?;
    let sink = actions.get(entity).ok()?;

    sink.play();
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .init_resource::<UiState>()
        .init_resource::<Story>()
        .init_resource::<StoryActions>()
        .init_resource::<CurrentAction>()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, input_box_ui.run_if(in_state(AppState::Input)))
        .add_systems(OnEnter(AppState::Loading), spawn_story_loader)
        .add_systems(
            Update,
            check_story_loader.run_if(in_state(AppState::Loading)),
        )
        .add_systems(OnExit(AppState::Loading), despawn_story_loader)
        .add_systems(OnEnter(AppState::Story), spawn_story_actions)
        .add_systems(OnExit(AppState::Story), despawn_actions)
        .add_systems(OnEnter(AppState::Story), spawn_subtitle_hud)
        .add_systems(OnExit(AppState::Story), despawn_subtitle_hud)
        .add_systems(
            PostUpdate,
            next_action_system.run_if(in_state(AppState::Story)),
        )
        .add_systems(Update, subtitle_system.run_if(in_state(AppState::Story)))
        .add_systems(Update, audio_system.run_if(in_state(AppState::Story)))
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Input,
    Loading,
    Story,
}

#[derive(Component, Debug)]
pub struct SubtitleHud;

#[derive(Component, Debug)]
pub struct SubtitleTextHud;

pub fn spawn_subtitle_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                ..default()
            },
            SubtitleHud {},
        ))
        .with_children(|parent| {
            parent
                .spawn((NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Percent(75.0),
                        margin: UiRect {
                            left: Val::Px(2.0),
                            right: Val::Px(2.0),
                            top: Val::Px(2.0),
                            bottom: Val::Px(20.0),
                        },
                        ..default()
                    },
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 40.0,
                                        color: Color::WHITE,
                                    },
                                )],
                                alignment: TextAlignment::Center,
                                linebreak_behavior: BreakLineOn::WordBoundary,
                            },
                            style: Style {
                                max_width: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: Color::rgb(0.0, 0.0, 0.0).into(),
                            ..default()
                        },
                        SubtitleTextHud {},
                    ));
                });
        });
}

pub fn despawn_subtitle_hud(mut commands: Commands, query: Query<Entity, With<SubtitleHud>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
