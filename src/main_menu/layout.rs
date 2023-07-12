use bevy::prelude::*;

use super::components::*;

use crate::styles;

pub fn build_main_menu(commands: &mut Commands) {
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
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    build_title(builder);
                });
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        row_gap: Val::Percent(5.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    build_menu(builder);
                });
        });
}

fn build_title(builder: &mut ChildBuilder) {
    builder.spawn(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Story".to_string(),
                    style: TextStyle {
                        font_size: 100.0,
                        color: styles::color::TEXT,
                        ..default()
                    },
                },
                TextSection {
                    value: "Gen".to_string(),
                    style: TextStyle {
                        font_size: 100.0,
                        color: styles::color::ACCENT,
                        ..default()
                    },
                },
            ],
            alignment: TextAlignment::Center,
            ..default()
        },
        ..default()
    });
}

fn build_menu(builder: &mut ChildBuilder) {
    builder
        .spawn((
            ButtonBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Px(200.0),
                    height: Val::Px(60.0),
                    ..Style::DEFAULT
                },
                background_color: BackgroundColor::from(styles::color::PRIMARY),
                ..default()
            },
            PlayButton {},
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Play",
                        TextStyle {
                            font_size: 40.0,
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
                background_color: BackgroundColor::from(styles::color::SECONDARY),
                ..default()
            },
            OptionsButton {},
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Options",
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
                background_color: BackgroundColor::from(styles::color::SECONDARY),
                ..default()
            },
            QuitButton {},
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Quit",
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
