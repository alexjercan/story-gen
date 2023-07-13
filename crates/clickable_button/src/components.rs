use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct ClickableButton {
    pub pressed_color: Color,
    pub hover_color: Color,
    pub normal_color: Color,
}
