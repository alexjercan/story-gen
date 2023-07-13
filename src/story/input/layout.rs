use super::components::*;
use crate::styles;
use bevy::prelude::*;
use clickable_button::ClickableButton;

pub fn build_input_menu(commands: &mut Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor::from(styles::color::BACKGROUND),
                visibility: Visibility::Visible,
                ..default()
            },
            InputMenu {},
        ))
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(50.0),
                        height: Val::Percent(50.0),
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

fn build_menu(builder: &mut ChildBuilder) {
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: BackgroundColor::from(styles::color::ACCENT),
            ..default()
        })
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexStart,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: BackgroundColor::from(styles::color::PRIMARY),
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: "".to_string(),
                                    style: TextStyle {
                                        font_size: 20.0,
                                        color: styles::color::TEXT,
                                        ..default()
                                    },
                                }],
                                alignment: TextAlignment::Left,
                                ..default()
                            },
                            ..default()
                        },
                        InputText {},
                    ));
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
                background_color: BackgroundColor::from(styles::color::PRIMARY),
                ..default()
            },
            ClickableButton {
                pressed_color: styles::color::FOCUS,
                hover_color: styles::color::HOVER,
                normal_color: styles::color::PRIMARY,
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
