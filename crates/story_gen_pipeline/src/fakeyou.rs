use fakeyou_api::tts::*;
use fakeyou_api::util::tts::*;
use fakeyou_api::*;

use crate::{Error, PipelineResult};

fn name_to_id(name: &str) -> PipelineResult<&str> {
    match name {
        "Rick" => Ok("TM:ebgxj0j4fvzp"),
        "Morty" => Ok("TM:mcvca56k5d5e"),
        name => Err(Error::AudioError(format!("Unknown name: {}", name))),
    }
}

pub(crate) fn generate_audio(name: &str, prompt: &str) -> PipelineResult<Vec<u8>> {
    let auth = Auth::default();
    let fakeyou = FakeYou::new(auth, FAKEYOU_API_URL);

    let name = name_to_id(name)?;

    let inference_body = InferenceBody::new(name, prompt);

    fakeyou
        .create_tts_task(&inference_body)
        .map(|t| t.bytes)
        .map_err(|e| Error::AudioError(e.to_string()))
}
