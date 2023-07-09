use crate::AudioError;
use std::collections::HashMap;

use fakeyou_api::tts::*;
use fakeyou_api::util::tts::*;
use fakeyou_api::*;

#[derive(Debug, Default)]
pub(crate) struct FakeYouTTS {
    name_to_id: HashMap<String, String>,
}

impl FakeYouTTS {
    pub(crate) fn new(name_to_id: HashMap<String, String>) -> Self {
        Self { name_to_id }
    }

    pub(crate) fn generate(&self, name: &str, prompt: &str) -> Result<Vec<u8>, AudioError> {
        let auth = Auth::default();
        let fakeyou = FakeYou::new(auth, FAKEYOU_API_URL);

        let name = self
            .name_to_id
            .get(name)
            .ok_or(AudioError::UnknownName(name.to_string()))?;

        let inference_body = InferenceBody::new(name, prompt);

        fakeyou
            .create_tts_task(&inference_body)
            .map(|t| t.bytes)
            .map_err(|e| AudioError::FakeYouError(e.to_string()))
    }
}
