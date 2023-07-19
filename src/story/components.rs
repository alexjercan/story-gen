use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct SubtitleHud;

#[derive(Component, Debug)]
pub struct SubtitleTextHud;

#[derive(Component, Debug)]
pub struct StoryActionValue;

#[derive(Component, Debug, Deref, DerefMut)]
pub struct StoryActionTimer(pub Timer);

#[derive(Component, Debug, Deref, DerefMut)]
pub struct StoryActionSubtitle(pub String);

#[derive(Component, Debug)]
pub struct StoryActionAudio;
