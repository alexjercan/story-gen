use bevy::{
    asset::{AssetLoader, LoadedAsset},
    reflect::{TypePath, TypeUuid},
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, TypeUuid, TypePath, Clone)]
#[uuid = "499f98fd-3296-4978-b0ae-0d851b781071"]
pub struct StoryAsset {
    pub id: String,
    #[serde(default)]
    pub icon: Option<String>,
    pub name: String,
    pub system: String,
    pub voices: HashMap<String, String>,
}

pub struct StoryAssetLoader;

impl AssetLoader for StoryAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            let story: StoryAsset = ron::de::from_bytes(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(story));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["story.ron"]
    }
}
