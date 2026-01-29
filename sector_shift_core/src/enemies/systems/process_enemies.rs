use bevy::prelude::*;

use crate::enemies::{
    assets::EnemyAsset,
    resources::{EnemyDefinition, EnemyLibrary},
};

/// A system that processes loaded enemy assets and adds them to the EnemyLibrary
pub fn process_enemies(
    mut e_enemy_asset: MessageReader<AssetEvent<EnemyAsset>>,
    asset_server: Res<AssetServer>,
    a_enemies: Res<Assets<EnemyAsset>>,
    mut enemy_library: ResMut<EnemyLibrary>,
) {
    for event in e_enemy_asset.read() {
        if let AssetEvent::LoadedWithDependencies { id } = event {
            let index = enemy_library.loading.iter().position(|handle| handle.id() == *id);
            if let Some(index) = index {
                if let Some(enemy_asset) = a_enemies.get(*id) {
                    enemy_library.add(EnemyDefinition::from_asset(&asset_server, enemy_asset));
                    info!("Loaded enemy asset");
                }
                enemy_library.loading.swap_remove(index);
            }
        }
    }
}
