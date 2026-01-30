use bevy::prelude::*;
use thiserror::Error;

use crate::environment::assets::EnvObjAsset;

/// An intermediate step between an environment asset and a spawned environment object
pub struct EnvObjDefinition {
    /// The unique ID of the environment element
    pub id: String,
    /// The model of the environment element
    pub gltf: Handle<Gltf>,
    /// The default scene of the environment element
    pub scene: Handle<Scene>,
    /// Whether this object can be set on the map in the editor
    /// This should be set to `false` for global environment like the skybox
    pub placeable: bool,
}

impl EnvObjDefinition {
    /// Helper to convert from [`EnvObjAsset`] to [`EnvObjDefinition`]
    pub fn from_asset(gltfs: &Assets<Gltf>, asset: &EnvObjAsset) -> Result<Self, EnvObjDefinitionLoadError> {
        let gltf_handle = asset.gltf_handle.clone().ok_or(EnvObjDefinitionLoadError::MissingGltfHandle)?;
        let gltf = gltfs.get(&gltf_handle).ok_or(EnvObjDefinitionLoadError::BrokenGltfHandle)?;
        Ok(Self {
            id: asset.id.clone(),
            gltf: gltf_handle,
            scene: gltf.scenes.first().ok_or(EnvObjDefinitionLoadError::NoDefaultScene)?.clone(),
            placeable: asset.placeable,
        })
    }
}

#[derive(Error, Debug)]
pub enum EnvObjDefinitionLoadError {
    #[error("Unexpected internal error: there is no gltf handle")]
    MissingGltfHandle,
    #[error("Unexpected internal error: broken gltf handle")]
    BrokenGltfHandle,
    #[error("The model does not contain any scenes")]
    NoDefaultScene,
}
