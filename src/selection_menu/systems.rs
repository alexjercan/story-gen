use super::components::*;
use super::layout::*;
use super::resources::*;
use crate::assets::{loader::StoryAsset, StoryAssets};
use crate::AppState;
use bevy::prelude::*;
use bevy_mod_sysfail::*;

pub fn spawn_selection_menu(
    mut commands: Commands,
    assets: Res<StoryAssets>,
    story_assets: Res<Assets<StoryAsset>>,
    mut selected_story: ResMut<SelectedStory>,
) {
    let stories = assets
        .stories
        .iter()
        .map(|h| story_assets.get(h).expect("bevy asset loader failed"))
        .collect::<Vec<_>>();

    build_selection_menu(&mut commands, stories);

    selected_story.0 = None;
}

#[quick_sysfail]
pub fn despawn_selection_menu(
    mut commands: Commands,
    selection_menu_query: Query<Entity, With<SelectionMenu>>,
) {
    let selection_menu_entity = selection_menu_query.get_single().ok()?;

    commands.entity(selection_menu_entity).despawn_recursive();
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

#[quick_sysfail]
pub fn update_description_visibility(
    mut description_visibility_query: Query<&mut Visibility, With<DescriptionElement>>,
    selected_story: Res<SelectedStory>,
) {
    let mut visibility = description_visibility_query.get_single_mut().ok()?;

    *visibility = if selected_story.0.is_some() {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}

#[quick_sysfail]
pub fn update_icon_image(
    mut icon_image_query: Query<&mut UiImage, With<IconImage>>,
    selected_story: Res<SelectedStory>,
    assets: Res<StoryAssets>,
) {
    let mut image = icon_image_query.get_single_mut().ok()?;
    let selected_icon = selected_story.0.as_ref()?.icon.as_ref()?;
    let handle = assets.icons.get(selected_icon)?;

    image.texture = handle.clone();
}

#[quick_sysfail]
pub fn update_system_text(
    mut system_text_query: Query<&mut Text, With<SystemText>>,
    selected_story: Res<SelectedStory>,
) {
    let mut text = system_text_query.get_single_mut().ok()?;
    let selected_system = &selected_story.0.as_ref()?.system;

    text.sections[0].value = selected_system.to_string();
}

#[quick_sysfail]
pub fn update_voices_text(
    mut voices_text_query: Query<&mut Text, With<VoicesText>>,
    selected_story: Res<SelectedStory>,
) {
    let mut text = voices_text_query.get_single_mut().ok()?;
    let selected_voices = &selected_story.0.as_ref()?.voices;

    text.sections[0].value = selected_voices
        .iter()
        .map(|(name, id)| format!("- {} ({})", name, id))
        .collect::<Vec<_>>()
        .join("\n");
}

#[quick_sysfail]
pub fn update_next_visibility(
    mut next_visibility_query: Query<&mut Visibility, With<NextButton>>,
    selected_story: Res<SelectedStory>,
) {
    let mut visibility = next_visibility_query.get_single_mut().ok()?;

    *visibility = if selected_story.0.is_some() {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}

pub fn interact_with_back_button(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok(Interaction::Pressed) = button_query.get_single_mut() {
        app_state_next_state.set(AppState::MainMenu);
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
