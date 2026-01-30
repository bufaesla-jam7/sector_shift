use bevy::{platform::collections::HashMap, prelude::*};
use thiserror::Error;

use crate::enemies::assets::EnemyAsset;

/// An intermediate step between an enemy asset and a spawned enemy
pub struct EnemyDefinition {
    /// The unique ID of the enemy used by MapObject::Enemy
    pub id: String,
    /// The sprite handle for the enemy
    pub sprite: Handle<Image>,
    pub gltf: Handle<Gltf>,
    pub scene: Handle<Scene>,
    pub graph: Handle<AnimationGraph>,
    pub idle: AnimationNodeIndex,
    pub walking: AnimationNodeIndex,
}

impl EnemyDefinition {
    /// Helper to convert from [`EnemyAsset`] to [`EnemyDefinition`]
    pub fn from_asset(
        asset_server: &AssetServer,
        gltfs: &Assets<Gltf>,
        graphs: &mut Assets<AnimationGraph>,
        asset: &EnemyAsset,
    ) -> Result<Self, EnemyDefinitionLoadError> {
        let gltf_handle = asset.gltf_handle.clone().ok_or(EnemyDefinitionLoadError::MissingGltfHandle)?;
        let gltf = gltfs.get(&gltf_handle).ok_or(EnemyDefinitionLoadError::BrokenGltfHandle)?;
        let (names, clips): (Vec<_>, Vec<_>) = gltf.named_animations.iter().unzip();
        let (graph, indices) = AnimationGraph::from_clips(clips.iter().map(|h| (*h).clone()));
        let named_indices: HashMap<_, AnimationNodeIndex> =
            names.into_iter().map(|s| s.as_ref()).zip(indices).collect();
        let get = |s: &'static str| {
            named_indices.get(s).ok_or(EnemyDefinitionLoadError::MissingAnimation(s)).copied()
        };

        Ok(Self {
            id: asset.id.clone(),
            sprite: asset_server.load(&asset.sprite),
            gltf: asset_server.load(&asset.gltf),
            scene: gltf.scenes.first().ok_or(EnemyDefinitionLoadError::NoDefaultScene)?.clone(),
            graph: graphs.add(graph),
            idle: get("Idle")?,
            walking: get("RunForward")?,
        })
    }
}

#[derive(Error, Debug)]
pub enum EnemyDefinitionLoadError {
    #[error("Unexpected internal error: there is no gltf handle")]
    MissingGltfHandle,
    #[error("Unexpected internal error: broken gltf handle")]
    BrokenGltfHandle,
    #[error("The model does not contain any scenes")]
    NoDefaultScene,
    #[error("The model was expected to contain an animation with name {0}")]
    MissingAnimation(&'static str),
}
