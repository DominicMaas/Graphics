use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct TileAssets {
    #[asset(path = "tiles", collection(typed))]
    pub tiles: Vec<Handle<Image>>,
}
