use bevy::prelude::*;

use openai_api_rust::*;

#[derive(Resource, Debug)]
pub struct StoryGenAuth {
    pub auth: Option<Auth>,
}

impl Default for StoryGenAuth {
    fn default() -> Self {
        let auth = Auth::from_env().ok();

        Self { auth }
    }
}
