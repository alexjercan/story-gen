use super::components::*;
use super::events::*;
use super::layout::*;
use super::resources::*;
use super::StoryState;
use crate::pipeline::ActionStory;
use bevy::prelude::*;

pub fn spawn_subtitle_hud(mut commands: Commands) {
    build_subtitle_hud(&mut commands);
}

pub fn despawn_subtitle_hud(mut commands: Commands, query: Query<Entity, With<SubtitleHud>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn handle_created_action(
    mut ev_input_action_story: EventReader<InputActionStoryEvent>,
    mut story_actions: ResMut<StoryActions>,
) {
    for ev in ev_input_action_story.iter() {
        story_actions.push_back(ev.0.clone());
    }
}

pub fn spawn_story_actions(
    mut commands: Commands,
    mut actions: ResMut<StoryActions>,
    mut story_next_state: ResMut<NextState<StoryState>>,
) {
    match actions.pop_front() {
        Some(action) => {
            match action {
                ActionStory::EndOfStory => {
                    // TODO: maybe I want to handle this in another way
                    story_next_state.set(StoryState::EndOfStory);
                    return;
                }
                ActionStory::Comment { text } => {
                    commands.spawn((
                        StoryActionValue,
                        StoryActionTimer(Timer::from_seconds(5.0, TimerMode::Once)),
                        StoryActionSubtitle(format!("{}", text)),
                    ));
                }
                ActionStory::Say { name, text, tts } => {
                    commands.spawn((
                        StoryActionValue,
                        StoryActionTimer(Timer::from_seconds(5.0, TimerMode::Once)),
                        StoryActionSubtitle(format!("{}: {}", name, text)),
                    ));

                    if let Some(handle) = tts {
                        // TODO: How can I find the duration of the audio?
                        commands.spawn((
                            StoryActionValue,
                            StoryActionTimer(Timer::from_seconds(5.0, TimerMode::Once)),
                            StoryActionAudio,
                            AudioBundle {
                                source: handle,
                                settings: PlaybackSettings::ONCE.paused(),
                            },
                        ));
                    }
                }
            };

            story_next_state.set(StoryState::Playing);
        }
        None => {}
    }
}

pub fn check_story_action(
    mut commands: Commands,
    time: Res<Time>,
    mut actions: Query<(Entity, &mut StoryActionTimer)>,
    mut story_next_state: ResMut<NextState<StoryState>>,
) {
    let finished = actions
        .iter_mut()
        .all(|(_, mut timer)| timer.tick(time.delta()).finished());

    if finished {
        for (entity, _) in actions.iter() {
            commands.entity(entity).despawn();
        }

        story_next_state.set(StoryState::Spawn);
    }
}

pub fn subtitle_system(
    mut text_query: Query<&mut Text, With<SubtitleTextHud>>,
    actions: Query<&StoryActionSubtitle>,
) {
    let subtitle = actions.iter().fold(String::new(), |mut acc, action| {
        acc.push_str(&action.0);
        acc.push_str("\n");
        acc
    });

    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("{}", subtitle);
    }
}

pub fn subtitle_clean_system(mut text_query: Query<&mut Text, With<SubtitleTextHud>>) {
    for mut text in text_query.iter_mut() {
        text.sections[0].value = String::new();
    }
}

pub fn audio_system(actions: Query<&AudioSink, With<StoryActionAudio>>) {
    actions.iter().for_each(|sink| {
        sink.play();
    });
}

pub fn end_of_story(
    mut ev_created_end_of_story: EventWriter<CreatedEndOfStoryEvent>,
    mut story_next_state: ResMut<NextState<StoryState>>,
) {
    ev_created_end_of_story.send(CreatedEndOfStoryEvent);
    story_next_state.set(StoryState::Spawn);
}
