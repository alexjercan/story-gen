use bevy::prelude::*;

use super::ClickableButton;

pub fn interact_with_button(
    mut button_query: Query<
        (&Interaction, &ClickableButton, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    button_query.iter_mut().for_each(
        |(
            interaction,
            ClickableButton {
                pressed_color,
                hover_color,
                normal_color,
            },
            mut background_color,
        )| {
            match *interaction {
                Interaction::Pressed => {
                    *background_color = BackgroundColor::from(*pressed_color);
                }
                Interaction::Hovered => {
                    *background_color = BackgroundColor::from(*hover_color);
                }
                Interaction::None => {
                    *background_color = BackgroundColor::from(*normal_color);
                }
            }
        },
    );
}
