use std::collections::VecDeque;
use bevy::prelude::*;
use parser::Action;

#[derive(Resource, Default)]
pub struct ActionsQueue {
    pub actions: VecDeque<Action>,
}

#[derive(Resource, Default)]
pub struct SayQueue {
    pub say: VecDeque<Option<Handle<AudioSource>>>,
}
