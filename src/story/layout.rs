use bevy::{prelude::*, text::BreakLineOn};

use super::components::{SubtitleHud, SubtitleTextHud};

pub fn build_subtitle_hud(commands: &mut Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            SubtitleHud {},
        ))
        .with_children(|parent| {
            parent
                .spawn((NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Percent(75.0),
                        margin: UiRect {
                            left: Val::Px(2.0),
                            right: Val::Px(2.0),
                            top: Val::Px(2.0),
                            bottom: Val::Px(20.0),
                        },
                        ..default()
                    },
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "",
                                    TextStyle {
                                        font_size: 40.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                )],
                                alignment: TextAlignment::Center,
                                linebreak_behavior: BreakLineOn::WordBoundary,
                            },
                            style: Style {
                                max_width: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: Color::BLACK.into(),
                            ..default()
                        },
                        SubtitleTextHud {},
                    ));
                });
        });
}
