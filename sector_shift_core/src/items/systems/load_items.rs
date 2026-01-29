use bevy::prelude::*;

use crate::items::{assets::ItemAsset, resources::ItemLibrary};

/// A system that loads all item assets
pub fn load_items(asset_server: Res<AssetServer>, mut item_library: ResMut<ItemLibrary>) {
    // TODO: Probably change this to just use asset_server.load_folder()
    if let Ok(directory) = std::fs::read_dir(ItemAsset::ASSET_PATH) {
        for entry in directory {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some(ItemAsset::EXTENSION) {
                let handle: Handle<ItemAsset> = asset_server.load(path);
                item_library.loading.push(handle.clone());
            }
        }
    }
}
