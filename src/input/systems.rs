use super::components::*;
use super::events::*;
use super::layout::*;
use super::resources::*;
use bevy::prelude::*;
use bevy_mod_sysfail::*;

pub fn spawn_input_menu(mut commands: Commands) {
    build_input_menu(&mut commands);
}

pub fn despawn_input_menu(mut commands: Commands, hud_menu_query: Query<Entity, With<InputMenu>>) {
    if let Ok(hud_menu_entity) = hud_menu_query.get_single() {
        commands.entity(hud_menu_entity).despawn_recursive();
    }
}

pub fn handle_input_menu(
    mut ev_input_menu: EventReader<InputMenuEvent>,
    mut visible: ResMut<InputVisible>,
) {
    for ev in ev_input_menu.iter() {
        visible.visible = ev.0;
    }
}

pub fn show_input_menu(
    mut hud_menu_query: Query<&mut Visibility, With<InputMenu>>,
    visible: Res<InputVisible>,
) {
    if let Ok(mut hud_menu_visibility) = hud_menu_query.get_single_mut() {
        *hud_menu_visibility = visible.visible;
    }
}

#[quick_sysfail]
pub fn interact_with_input_text(
    mut evr_char: EventReader<ReceivedCharacter>,
    kbd: Res<Input<KeyCode>>,
    mut text_query: Query<&mut Text, With<InputText>>,
) {
    let mut text = text_query.get_single_mut().ok()?;

    if kbd.just_pressed(KeyCode::Back) {
        text.sections[0].value.pop();
    }
    for ev in evr_char.iter() {
        if !ev.char.is_control() {
            text.sections[0].value.push(ev.char);
        }
    }
}

#[quick_sysfail]
pub fn submit_input_text(
    button_query: Query<&Interaction, (Changed<Interaction>, With<ContinueButton>)>,
    kbd: Res<Input<KeyCode>>,
    mut text_query: Query<&mut Text, With<InputText>>,
    mut visible: ResMut<InputVisible>,
    mut ev_text: EventWriter<CreatedTextEvent>,
) {
    if Some(&Interaction::Pressed) == button_query.get_single().ok()
        || kbd.just_pressed(KeyCode::Return)
    {
        let mut text = text_query.get_single_mut().ok()?;
        ev_text.send(CreatedTextEvent(text.sections[0].value.clone()));
        text.sections[0].value.clear();
        visible.visible = Visibility::Hidden;
    }
}
