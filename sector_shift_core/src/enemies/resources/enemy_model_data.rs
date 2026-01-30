use bevy::{platform::collections::HashMap, prelude::*};
use thiserror::Error;

use crate::enemies::resources::EnemyDefinition;

/// An intermediate step between an enemy asset and a spawned enemy
pub struct EnemyModelData {
    pub scene: Handle<Scene>,
    pub graph: Handle<AnimationGraph>,
    pub idle: AnimationNodeIndex,
    pub walking: AnimationNodeIndex,
}

impl EnemyModelData {
    /// Helper to create a [`EnemyModelData`] from a [`EnemyDefinition`] once the contained
    /// [`Gltf`] model is loaded
    pub fn from_asset(
        gltfs: &Assets<Gltf>,
        graphs: &mut Assets<AnimationGraph>,
        asset: &EnemyDefinition,
    ) -> Result<Self, EnemyModelDataLoadError> {
        let gltf = gltfs.get(&asset.gltf).ok_or(EnemyModelDataLoadError::BrokenGltfHandle)?;
        let (names, clips): (Vec<_>, Vec<_>) = gltf.named_animations.iter().unzip();
        let (graph, indices) = AnimationGraph::from_clips(clips.iter().map(|h| (*h).clone()));
        let named_indices: HashMap<_, AnimationNodeIndex> =
            names.into_iter().map(|s| s.as_ref()).zip(indices).collect();
        let get = |s: &'static str| {
            named_indices.get(s).ok_or(EnemyModelDataLoadError::MissingAnimation(s)).copied()
        };

        Ok(Self {
            scene: gltf.scenes.first().ok_or(EnemyModelDataLoadError::NoDefaultScene)?.clone(),
            graph: graphs.add(graph),
            idle: get("Idle")?,
            walking: get("RunForward")?,
        })
    }
}

#[derive(Error, Debug)]
pub enum EnemyModelDataLoadError {
    #[error("Unexpected internal error: broken gltf handle")]
    BrokenGltfHandle,
    #[error("The model does not contain any scenes")]
    NoDefaultScene,
    #[error("The model was expected to contain an animation with name {0}")]
    MissingAnimation(&'static str),
}
