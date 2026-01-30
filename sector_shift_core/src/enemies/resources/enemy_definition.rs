use bevy::prelude::*;

use crate::enemies::assets::EnemyAsset;

/// An intermediate step between an enemy asset and a spawned enemy
pub struct EnemyDefinition {
    /// The unique ID of the enemy used by MapObject::Enemy
    pub id: String,
    /// The sprite handle for the enemy
    pub sprite: Handle<Image>,
    pub gltf: Handle<Gltf>,
}

impl EnemyDefinition {
    /// Helper to convert from EnemyAsset to EnemyDefinition
    pub fn from_asset(asset_server: &AssetServer, asset: &EnemyAsset) -> Self {
        Self {
            id: asset.id.clone(),
            sprite: asset_server.load(&asset.sprite),
            gltf: asset_server.load(&asset.gltf),
        }
    }
}
