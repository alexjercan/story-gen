use crate::assets::loader::StoryAsset;
use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct SelectionMenu;

#[derive(Component, Debug)]
pub struct SelectionButton {
    pub story: StoryAsset,
}

#[derive(Component, Debug)]
pub struct BackButton;

#[derive(Component, Debug)]
pub struct NextButton;

#[derive(Component, Debug)]
pub struct DescriptionElement;

#[derive(Component, Debug)]
pub struct IconImage;

#[derive(Component, Debug)]
pub struct SystemText;

#[derive(Component, Debug)]
pub struct VoicesText;
