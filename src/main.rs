use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
};

use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
    text::BreakLineOn,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_mod_sysfail::*;

use futures_lite::future;
use story_gen_pipeline::*;

const SYSTEM: &str = r#"""You are given the formal language for describing a story. The description of
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

#[derive(Default, Resource)]
pub struct StoryPipeline(pub Arc<Mutex<Pipeline>>);

#[derive(Default, Resource, Deref, DerefMut)]
pub struct Story(pub Vec<StoryAction>);

#[derive(Default, Resource, Deref, DerefMut)]
pub struct StoryActions(pub VecDeque<StoryAction>);

pub fn input_box_ui(
    mut state: ResMut<UiState>,
    pipeline: Res<StoryPipeline>,
    mut story: ResMut<Story>,
    mut story_actions: ResMut<StoryActions>,
    mut contexts: EguiContexts,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut pipeline_next_state: ResMut<NextState<PipelineState>>,
    mut story_next_state: ResMut<NextState<StoryState>>,
) {
    let mut submit = false;
    let mut replay = false;
    let mut continue_story = false;

    egui::TopBottomPanel::bottom("input_box").show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered_justified(|ui| {
            ui.code_editor(&mut state.input_box);
            ui.separator();
            ui.horizontal(|ui| {
                submit = ui.button("Submit").clicked();
                ui.separator();
                replay = ui.button("Replay").clicked();
                ui.separator();
                continue_story = ui.button("Continue").clicked();
            });
        });
    });

    if submit {
        contexts.ctx_mut().request_repaint();

        story.0.clear();

        let story = state.input_box.clone();
        let mut pipeline = pipeline.0.lock().unwrap();
        pipeline.clear();
        pipeline.push_back(&story);

        state.input_box.clear();

        app_state_next_state.set(AppState::Story);
        pipeline_next_state.set(PipelineState::Spawn);
        story_next_state.set(StoryState::Spawn);
    }

    if replay {
        contexts.ctx_mut().request_repaint();

        story_actions.extend(story.0.iter().cloned());

        state.input_box.clear();

        app_state_next_state.set(AppState::Story);
        pipeline_next_state.set(PipelineState::Idle);
        story_next_state.set(StoryState::Spawn);
    }

    if continue_story {
        contexts.ctx_mut().request_repaint();

        let story = state.input_box.clone();
        let mut pipeline = pipeline.0.lock().unwrap();
        pipeline.push_back(&story);

        state.input_box.clear();

        app_state_next_state.set(AppState::Story);
        pipeline_next_state.set(PipelineState::Spawn);
        story_next_state.set(StoryState::Spawn);
    }
}

#[derive(Component)]
pub struct StoryActionLoader(pub Task<Option<PipelineResult<StoryAction>>>);

pub fn spawn_story_loader(
    mut commands: Commands,
    pipeline: Res<StoryPipeline>,
    mut pipeline_next_state: ResMut<NextState<PipelineState>>,
) {
    let thread_pool = AsyncComputeTaskPool::get();
    let pipeline = pipeline.0.clone();
    let task = thread_pool.spawn(async move {
        let mut pipeline = pipeline.lock().unwrap();
        pipeline.next()
    });

    commands.spawn(StoryActionLoader(task));

    pipeline_next_state.set(PipelineState::Loading);
}

pub fn check_story_loader(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut StoryActionLoader)>,
    mut story: ResMut<Story>,
    mut story_actions: ResMut<StoryActions>,
    mut pipeline_next_state: ResMut<NextState<PipelineState>>,
) {
    let (entity, mut task) = tasks.get_single_mut().unwrap();

    if let Some(action) = future::block_on(future::poll_once(&mut task.0)) {
        match action {
            Some(Ok(action)) => {
                story.0.push(action.clone());
                story_actions.0.push_back(action);
                pipeline_next_state.set(PipelineState::Spawn);
            }
            Some(Err(err)) => {
                println!("Error: {}", err.to_string());
                pipeline_next_state.set(PipelineState::Spawn);
            }
            None => {
                println!("No more actions");
                pipeline_next_state.set(PipelineState::Idle);
            }
        }

        commands.entity(entity).despawn();
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
pub struct StoryActionTimer(pub Timer);

#[derive(Component, Debug)]
pub struct StoryActionSubtitle(pub String);

#[derive(Component, Debug)]
pub struct StoryActionAudio;

pub fn spawn_story_actions(
    mut commands: Commands,
    mut actions: ResMut<StoryActions>,
    mut assets: ResMut<Assets<AudioSource>>,
    mut story_next_state: ResMut<NextState<StoryState>>,
) {
    match actions.0.pop_front() {
        Some(action) => {
            match action {
                StoryAction::Comment(text) => commands.spawn((
                    StoryActionValue,
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

                    // TODO: I guess I can also have one entity for each action type
                    commands.spawn((
                        StoryActionValue,
                        StoryActionTimer(Timer::from_seconds(duration, TimerMode::Once)),
                        StoryActionSubtitle(format!("{}: {}", name, text)),
                        StoryActionAudio,
                        AudioBundle {
                            source: handle,
                            settings: PlaybackSettings::ONCE.paused(),
                        },
                    ))
                }
            };

            story_next_state.set(StoryState::Loading);
        }
        None => {
            story_next_state.set(StoryState::Idle);
        }
    }
}

pub fn despawn_actions(mut commands: Commands, actions: Query<Entity, With<StoryActionValue>>) {
    for entity in actions.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn check_story_action(
    mut commands: Commands,
    time: Res<Time>,
    mut actions: Query<(Entity, &mut StoryActionTimer)>,
    mut story_next_state: ResMut<NextState<StoryState>>,
) {
    let (entity, mut timer) = actions.get_single_mut().unwrap();

    if timer.0.tick(time.delta()).just_finished() {
        commands.entity(entity).despawn();

        story_next_state.set(StoryState::Spawn);
    }
}

#[quick_sysfail]
pub fn subtitle_system(
    mut text_query: Query<&mut Text, With<SubtitleTextHud>>,
    actions: Query<&StoryActionSubtitle>,
) {
    let subtitle = actions.get_single().ok()?;

    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("{}", subtitle.0);
    }
}

#[quick_sysfail]
pub fn audio_system(actions: Query<&AudioSink, With<StoryActionAudio>>) {
    let sink = actions.get_single().ok()?;

    sink.play();
}

pub fn resume_story(mut story_next_state: ResMut<NextState<StoryState>>) {
    story_next_state.set(StoryState::Spawn);
}

pub fn conclude_story(mut app_state_next_state: ResMut<NextState<AppState>>) {
    app_state_next_state.set(AppState::Input);
}

fn main() {
    App::new()
        // States
        .add_state::<AppState>()
        .add_state::<PipelineState>()
        .add_state::<StoryState>()
        // Resources
        .init_resource::<UiState>()
        .insert_resource(StoryPipeline(Arc::new(Mutex::new(Pipeline::new(
            SYSTEM,
            HashMap::from_iter(vec![
                ("Rick".to_string(), "TM:ebgxj0j4fvzp".to_string()),
                ("Morty".to_string(), "TM:mcvca56k5d5e".to_string()),
            ]),
        )))))
        .init_resource::<Story>()
        .init_resource::<StoryActions>()
        // Plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        // Systems
        .add_systems(Startup, setup)
        .add_systems(Update, input_box_ui.run_if(in_state(AppState::Input)))
        // Pipeline Systems
        .add_systems(
            Update,
            spawn_story_loader
                .run_if(in_state(AppState::Story).and_then(in_state(PipelineState::Spawn))),
        )
        .add_systems(
            Update,
            check_story_loader
                .run_if(in_state(AppState::Story).and_then(in_state(PipelineState::Loading))),
        )
        .add_systems(OnExit(AppState::Story), despawn_story_loader)
        // Story Systems
        .add_systems(
            Update,
            spawn_story_actions
                .run_if(in_state(AppState::Story).and_then(in_state(StoryState::Spawn))),
        )
        .add_systems(
            Update,
            subtitle_system
                .run_if(in_state(AppState::Story).and_then(in_state(StoryState::Loading))),
        )
        .add_systems(
            Update,
            audio_system.run_if(in_state(AppState::Story).and_then(in_state(StoryState::Loading))),
        )
        .add_systems(
            PostUpdate,
            check_story_action
                .run_if(in_state(AppState::Story).and_then(in_state(StoryState::Loading))),
        )
        .add_systems(OnExit(AppState::Story), despawn_actions)
        .add_systems(OnEnter(AppState::Story), spawn_subtitle_hud)
        .add_systems(OnExit(AppState::Story), despawn_subtitle_hud)
        // Conclude Story
        .add_systems(
            Update,
            resume_story.run_if(
                in_state(AppState::Story).and_then(
                    not(in_state(PipelineState::Idle)).and_then(in_state(StoryState::Idle)),
                ),
            ),
        )
        .add_systems(
            Update,
            conclude_story.run_if(
                in_state(AppState::Story)
                    .and_then(in_state(PipelineState::Idle).and_then(in_state(StoryState::Idle))),
            ),
        )
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Input,
    Story,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PipelineState {
    #[default]
    Spawn,
    Loading,
    Idle,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum StoryState {
    #[default]
    Spawn,
    Loading,
    Idle,
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
