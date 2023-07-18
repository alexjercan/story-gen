use super::loader::*;
use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource, Debug)]
pub struct StoryAssets {
    #[asset(path = "story", collection(typed))]
    pub stories: Vec<Handle<StoryAsset>>,
    #[asset(path = "icons", collection(typed, mapped))]
    pub icons: HashMap<String, Handle<Image>>
}
