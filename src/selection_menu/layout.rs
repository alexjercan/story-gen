use super::components::*;
use crate::{assets::loader::StoryAsset, styles};
use bevy::prelude::*;
use clickable_button::ClickableButton;

pub fn build_selection_menu(commands: &mut Commands, stories: Vec<&StoryAsset>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            SelectionMenu,
        ))
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(15.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor::from(styles::color::BACKGROUND),
                    ..default()
                })
                .with_children(|builder| {
                    build_title(builder);
                });
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(75.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(5.0),
                        padding: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    background_color: BackgroundColor::from(styles::color::TEXT),
                    ..default()
                })
                .with_children(|builder| {
                    build_content(builder, stories);
                });
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(10.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: BackgroundColor::from(styles::color::BACKGROUND),
                    ..default()
                })
                .with_children(|builder| {
                    build_control(builder);
                });
        });
}

fn build_title(builder: &mut ChildBuilder) {
    builder.spawn(TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "Selection Menu".to_string(),
                style: TextStyle {
                    font_size: 50.0,
                    color: styles::color::TEXT,
                    ..default()
                },
            }],
            alignment: TextAlignment::Center,
            ..default()
        },
        ..default()
    });
}

fn build_content(builder: &mut ChildBuilder, stories: Vec<&StoryAsset>) {
    builder
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                height: Val::Percent(100.0),
                width: Val::Percent(30.0),
                row_gap: Val::Px(5.0),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            background_color: BackgroundColor::from(styles::color::BACKGROUND),
            ..default()
        })
        .with_children(|builder| {
            stories.into_iter().for_each(|story| {
                build_story_element(builder, story);
            });
        });
    builder
        .spawn((
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(70.0),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                visibility: Visibility::Hidden,
                background_color: BackgroundColor::from(styles::color::BACKGROUND),
                ..default()
            },
            DescriptionElement,
        ))
        .with_children(|builder| {
            build_description_element(builder);
        });
}

fn build_story_element(builder: &mut ChildBuilder, story: &StoryAsset) {
    builder
        .spawn((
            ButtonBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Px(60.0),
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
            SelectionButton {
                story: story.clone(),
            },
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        story.name.clone(),
                        TextStyle {
                            font_size: 20.0,
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

fn build_description_element(builder: &mut ChildBuilder) {
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                ImageBundle {
                    style: Style {
                        max_height: Val::Px(200.0),
                        aspect_ratio: Some(1.0),
                        ..default()
                    },
                    ..default()
                },
                IconImage,
            ));
        });
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "System Prompt",
                        TextStyle {
                            font_size: 20.0,
                            color: styles::color::ACCENT,
                            ..default()
                        },
                    )],
                    alignment: TextAlignment::Left,
                    ..default()
                },
                ..default()
            },));
        });
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "",
                            TextStyle {
                                font_size: 10.0,
                                color: styles::color::TEXT,
                                ..default()
                            },
                        )],
                        alignment: TextAlignment::Left,
                        ..default()
                    },
                    ..default()
                },
                SystemText,
            ));
        });

    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Voices",
                        TextStyle {
                            font_size: 20.0,
                            color: styles::color::ACCENT,
                            ..default()
                        },
                    )],
                    alignment: TextAlignment::Left,
                    ..default()
                },
                ..default()
            },));
        });
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "",
                            TextStyle {
                                font_size: 10.0,
                                color: styles::color::TEXT,
                                ..default()
                            },
                        )],
                        alignment: TextAlignment::Left,
                        ..default()
                    },
                    ..default()
                },
                VoicesText,
            ));
        });
}

fn build_control(builder: &mut ChildBuilder) {
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
            ClickableButton {
                pressed_color: styles::color::FOCUS,
                hover_color: styles::color::HOVER,
                normal_color: styles::color::SECONDARY,
            },
            BackButton,
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "back",
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
                background_color: BackgroundColor::from(styles::color::PRIMARY),
                visibility: Visibility::Hidden,
                ..default()
            },
            ClickableButton {
                pressed_color: styles::color::FOCUS,
                hover_color: styles::color::HOVER,
                normal_color: styles::color::PRIMARY,
            },
            NextButton,
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "next",
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
