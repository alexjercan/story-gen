use super::components::*;
use super::layout::*;
use super::resources::*;
use crate::loader::StoryAsset;
use crate::resources::Stories;
use crate::AppState;
use bevy::prelude::*;

pub fn spawn_selection_menu(
    mut commands: Commands,
    stories: Res<Stories>,
    story_assets: Res<Assets<StoryAsset>>,
) {
    // TODO: handle the error
    let stories = stories
        .stories
        .iter()
        .map(|h| story_assets.get(h).unwrap())
        .collect::<Vec<_>>();

    build_selection_menu(&mut commands, stories);
}

pub fn despawn_selection_menu(
    mut commands: Commands,
    selection_menu_query: Query<Entity, With<SelectionMenu>>,
) {
    if let Ok(selection_menu_entity) = selection_menu_query.get_single() {
        commands.entity(selection_menu_entity).despawn_recursive();
    }
}

pub fn interact_with_selection_button(
    mut button_query: Query<
        (&Interaction, &SelectionButton),
        (Changed<Interaction>, With<SelectionButton>),
    >,
    mut selected_story: ResMut<SelectedStory>,
) {
    if let Ok((Interaction::Pressed, SelectionButton { story })) = button_query.get_single_mut() {
        selected_story.0 = Some(story.clone());
    }
}

pub fn update_system_text(
    mut system_text_query: Query<&mut Text, With<SystemText>>,
    selected_story: Res<SelectedStory>,
) {
    if let Ok(mut text) = system_text_query.get_single_mut() {
        text.sections[0].value = selected_story
            .0
            .as_ref()
            .map(|s| s.system.to_string())
            .unwrap_or_default();
    }
}

pub fn update_voices_text(
    mut voices_text_query: Query<&mut Text, With<VoicesText>>,
    selected_story: Res<SelectedStory>,
) {
    if let Ok(mut text) = voices_text_query.get_single_mut() {
        text.sections[0].value = selected_story
            .0
            .as_ref()
            .map(|s| {
                s.voices
                    .iter()
                    .map(|(name, id)| format!("- {} ({})", name, id))
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .unwrap_or_default();
    }
}

pub fn update_next_visibility(
    mut next_visibility_query: Query<&mut Visibility, With<NextButton>>,
    selected_story: Res<SelectedStory>,
) {
    if let Ok(mut visibility) = next_visibility_query.get_single_mut() {
        *visibility = if selected_story.0.is_some() {
            Visibility::Visible
        } else {
            Visibility::Hidden
        }
    }
}

pub fn interact_with_back_button(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut selected_story: ResMut<SelectedStory>,
) {
    if let Ok(Interaction::Pressed) = button_query.get_single_mut() {
        app_state_next_state.set(AppState::MainMenu);
        selected_story.0 = None;
    }
}

pub fn interact_with_next_button(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<NextButton>)>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok(Interaction::Pressed) = button_query.get_single_mut() {
        app_state_next_state.set(AppState::Story)
    }
}
