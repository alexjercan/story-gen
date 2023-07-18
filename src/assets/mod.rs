pub mod loader;
mod resources;

use crate::AppState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use loader::*;
pub use resources::StoryAssets;

pub struct AssetsLoaderPlugin;

impl Plugin for AssetsLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<StoryAsset>()
            .add_asset_loader(StoryAssetLoader)
            .add_loading_state(
                LoadingState::new(AppState::AssetLoading).continue_to_state(AppState::MainMenu),
            )
            .add_collection_to_loading_state::<_, StoryAssets>(AppState::AssetLoading);
    }
}
