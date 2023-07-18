use bevy::prelude::Component;
use crate::loader::StoryAsset;

#[derive(Component, Debug)]
pub struct SelectionMenu {}

#[derive(Component, Debug)]
pub struct SelectionButton { pub story: StoryAsset }

#[derive(Component, Debug)]
pub struct BackButton {}

#[derive(Component, Debug)]
pub struct NextButton {}

#[derive(Component, Debug)]
pub struct SystemText {}

#[derive(Component, Debug)]
pub struct VoicesText {}
