use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "say")]
    Say { name: String, text: String },
    #[serde(rename = "comment")]
    Comment { text: String },
}

#[derive(Resource, Default)]
pub struct ActionsQueue {
    pub actions: VecDeque<Action>,
}

#[derive(Resource, Default)]
pub struct SayQueue {
    pub say: VecDeque<Option<Handle<AudioSource>>>,
}
