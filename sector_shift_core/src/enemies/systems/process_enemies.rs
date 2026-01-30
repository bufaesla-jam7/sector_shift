use bevy::prelude::*;

use crate::enemies::{
    assets::EnemyAsset,
    resources::{EnemyDefinition, EnemyLibrary},
};

/// A system that processes loaded enemy assets and adds them to the EnemyLibrary
pub fn process_enemies(
    asset_server: Res<AssetServer>,
    gltfs: Res<Assets<Gltf>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    a_enemies: Res<Assets<EnemyAsset>>,
    mut enemy_library: ResMut<EnemyLibrary>,
) {
    if enemy_library.loading_finished {
        for handle in std::mem::take(&mut enemy_library.loading) {
            if let Some(enemy_asset) = a_enemies.get(&handle) {
                match EnemyDefinition::from_asset(&asset_server, &gltfs, &mut graphs, enemy_asset) {
                    Ok(definition) => {
                        info!("Loaded enemy asset with id {}", definition.id);
                        enemy_library.add(definition);
                    },
                    Err(e) => {
                        warn!("Failed to load enemy asset: {e}")
                    },
                }
            }
        }
    }
}
