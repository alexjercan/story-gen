use std::collections::HashMap;

use super::StoryAction;
use ::bevy::prelude::*;

// TODO: This should be an asset prolly combine with the one from chatgpt (PROMPT)
#[derive(Resource, Debug, Clone)]
pub struct TTSOptions {
    pub names: HashMap<String, String>,
}

impl Default for TTSOptions {
    fn default() -> Self {
        Self {
            names: HashMap::from_iter(vec![
                ("Rick".to_string(), "TM:ebgxj0j4fvzp".to_string()),
                ("Morty".to_string(), "TM:mcvca56k5d5e".to_string()),
            ]),
        }
    }
}

#[derive(Resource, Default, Debug, Deref, DerefMut)]
pub struct StoryActions(pub Vec<StoryAction>);
