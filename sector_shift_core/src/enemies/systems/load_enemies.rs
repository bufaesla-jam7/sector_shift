use bevy::prelude::*;

use crate::enemies::{assets::EnemyAsset, resources::EnemyLibrary};

/// A system that loads all enemy assets
pub fn load_enemies(asset_server: Res<AssetServer>, mut enemy_library: ResMut<EnemyLibrary>) {
    // TODO: Probably change this to just use asset_server.load_folder()
    if let Ok(directory) = std::fs::read_dir(EnemyAsset::ASSET_PATH) {
        for entry in directory {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some(EnemyAsset::EXTENSION) {
                let handle: Handle<EnemyAsset> = asset_server.load(path);
                enemy_library.loading.push(handle.clone());
            }
        }
    }
}
