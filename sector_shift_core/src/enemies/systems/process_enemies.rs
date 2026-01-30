use bevy::prelude::*;

use crate::enemies::{
    assets::EnemyAsset,
    resources::{EnemyDefinition, EnemyLibrary, EnemyModelData},
};

/// A system that processes loaded enemy assets and adds them to the EnemyLibrary
pub fn process_enemies(
    mut e_enemy_asset: MessageReader<AssetEvent<EnemyAsset>>,
    asset_server: Res<AssetServer>,
    gltfs: Res<Assets<Gltf>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    a_enemies: Res<Assets<EnemyAsset>>,
    mut enemy_library: ResMut<EnemyLibrary>,
) {
    for event in e_enemy_asset.read() {
        if let AssetEvent::LoadedWithDependencies { id } = event {
            let index = enemy_library.loading_assets.iter().position(|handle| handle.id() == *id);
            if let Some(index) = index {
                if let Some(enemy_asset) = a_enemies.get(*id) {
                    enemy_library
                        .loading_models
                        .push(EnemyDefinition::from_asset(&asset_server, enemy_asset));
                    info!("Loaded enemy asset");
                }
                enemy_library.loading_assets.swap_remove(index);
            }
        }
    }
    for index in (0..enemy_library.loading_models.len()).rev() {
        if asset_server.is_loaded_with_dependencies(&enemy_library.loading_models[index].gltf) {
            let definition = enemy_library.loading_models.swap_remove(index);
            let model_data = match EnemyModelData::from_asset(&gltfs, &mut graphs, &definition) {
                Ok(md) => md,
                Err(e) => {
                    error!("Failed to load enemy {}: {e}", definition.id);
                    continue;
                },
            };
            enemy_library.add(definition, model_data);
        }
    }
}
