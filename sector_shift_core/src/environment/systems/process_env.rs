use bevy::prelude::*;

use crate::environment::{
    assets::EnvObjAsset,
    resources::{EnvObjDefinition, EnvObjLibrary},
};

/// A system that processes loaded environment assets and adds them to the [`EnvObjLibrary`]
pub fn process_env_objs(
    mut e_env_asset: MessageReader<AssetEvent<EnvObjAsset>>,
    gltfs: Res<Assets<Gltf>>,
    a_env_objs: Res<Assets<EnvObjAsset>>,
    mut env_library: ResMut<EnvObjLibrary>,
) {
    for event in e_env_asset.read() {
        if let AssetEvent::LoadedWithDependencies { id } = event {
            let index = env_library.loading.iter().position(|handle| handle.id() == *id);
            if let Some(index) = index {
                if let Some(env_asset) = a_env_objs.get(*id) {
                    match EnvObjDefinition::from_asset(&gltfs, env_asset) {
                        Ok(definition) => {
                            info!("Loaded environment object asset with id {}", definition.id);
                            env_library.add(definition);
                        },
                        Err(e) => {
                            warn!("Failed to load environment object asset: {e}")
                        },
                    }
                }
                env_library.loading.swap_remove(index);
            }
        }
    }
}
