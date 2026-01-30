use bevy::prelude::*;

use crate::environment::{
    assets::EnvObjAsset,
    resources::{EnvObjDefinition, EnvObjLibrary},
};

/// A system that processes loaded environment assets and adds them to the [`EnvObjLibrary`]
pub fn process_env_objs(
    gltfs: Res<Assets<Gltf>>,
    a_env_objs: Res<Assets<EnvObjAsset>>,
    mut env_library: ResMut<EnvObjLibrary>,
) {
    if env_library.loading_finished {
        for handle in std::mem::take(&mut env_library.loading) {
            if let Some(env_asset) = a_env_objs.get(&handle) {
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
        }
    }
}
