use super::components::*;
use super::events::*;
use super::layout::*;
use crate::story::StoryState;
use bevy::prelude::*;

pub fn spawn_input_menu(mut commands: Commands) {
    build_input_menu(&mut commands);
}

pub fn despawn_input_menu(mut commands: Commands, hud_menu_query: Query<Entity, With<InputMenu>>) {
    if let Ok(hud_menu_entity) = hud_menu_query.get_single() {
        commands.entity(hud_menu_entity).despawn_recursive();
    }
}

pub fn show_input_menu(mut hud_menu_query: Query<&mut Visibility, With<InputMenu>>) {
    if let Ok(mut hud_menu_visibility) = hud_menu_query.get_single_mut() {
        *hud_menu_visibility = Visibility::Visible;
    }
}

pub fn hide_input_menu(mut hud_menu_query: Query<&mut Visibility, With<InputMenu>>) {
    if let Ok(mut hud_menu_visibility) = hud_menu_query.get_single_mut() {
        *hud_menu_visibility = Visibility::Hidden;
    }
}

pub fn interact_with_input_text(
    mut evr_char: EventReader<ReceivedCharacter>,
    kbd: Res<Input<KeyCode>>,
    mut text_query: Query<&mut Text, With<InputText>>,
) {
    let mut text = text_query.single_mut();

    if kbd.just_pressed(KeyCode::Back) {
        text.sections[0].value.pop();
    }
    for ev in evr_char.iter() {
        if !ev.char.is_control() {
            text.sections[0].value.push(ev.char);
        }
    }
}

pub fn submit_input_text(
    button_query: Query<&Interaction, With<ContinueButton>>,
    kbd: Res<Input<KeyCode>>,
    mut text_query: Query<&mut Text, With<InputText>>,
    mut story_state_next_state: ResMut<NextState<StoryState>>,
    mut ev_text: EventWriter<InputTextEvent>,
) {
    if Some(&Interaction::Pressed) == button_query.get_single().ok()
        || kbd.just_pressed(KeyCode::Return)
    {
        let mut text = text_query.single_mut();
        ev_text.send(InputTextEvent(text.sections[0].value.clone()));
        text.sections[0].value.clear();
        story_state_next_state.set(StoryState::Simulation);
    }
}
