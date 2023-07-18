use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use super::loader::*;

#[derive(AssetCollection, Resource)]
pub struct StoryAssets {
    #[asset(path = "story", collection(typed))]
    pub stories: Vec<Handle<StoryAsset>>,
}

