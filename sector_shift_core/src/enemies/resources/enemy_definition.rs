use bevy::prelude::*;

use crate::enemies::assets::EnemyAsset;

pub struct EnemyDefinition {
    pub id: String,
    pub sprite: Handle<Image>,
}

impl EnemyDefinition {
    pub fn from_asset(asset_server: &AssetServer, asset: &EnemyAsset) -> Self {
        Self {
            id: asset.id.clone(),
            sprite: asset_server.load(&asset.sprite),
        }
    }
}
