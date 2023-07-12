use bevy::prelude::*;

use super::components::*;

use crate::styles;

pub fn build_hud_menu(commands: &mut Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor::from(styles::color::BACKGROUND),
                visibility: Visibility::Hidden,
                ..default()
            },
            HudMenu {},
        ))
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(50.0),
                        height: Val::Percent(50.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    build_menu(builder);
                });
        });
}

fn build_menu(builder: &mut ChildBuilder) {
    builder.spawn(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "The input menu is under construction!".to_string(),
                    style: TextStyle {
                        font_size: 40.0,
                        color: styles::color::TEXT,
                        ..default()
                    },
                },
            ],
            alignment: TextAlignment::Center,
            ..default()
        },
        ..default()
    });

    builder
        .spawn((
            ButtonBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Px(150.0),
                    height: Val::Px(40.0),
                    ..Style::DEFAULT
                },
                background_color: BackgroundColor::from(styles::color::PRIMARY),
                ..default()
            },
            ContinueButton {},
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Continue",
                        TextStyle {
                            font_size: 30.0,
                            color: styles::color::TEXT,
                            ..default()
                        },
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        });
}
