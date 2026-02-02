mod movement_info;
pub use self::movement_info::*;

mod action_info;
pub use self::action_info::*;

use bevy::{platform::collections::HashMap, prelude::*};
use thiserror::Error;

use crate::{enemies::assets::EnemyAsset, prelude::Enemy};

/// An intermediate step between an enemy asset and a spawned enemy
#[derive(Reflect)]
pub struct EnemyDefinition {
    /// The unique ID of the enemy used by MapObject::Enemy
    pub id: String,
    /// The sprite handle for the enemy
    pub sprite: Handle<Image>,
    /// [`Enemy`] component to be directly inserted on each enemy entity
    pub attributes: Enemy,
    pub gltf: Handle<Gltf>,
    pub scene: Handle<Scene>,
    pub graph: Handle<AnimationGraph>,
    pub animation_transition_duration_ms: u64,
    #[reflect(ignore)]
    pub movement_animation_info: EnemyMovementAnimationInfo,
    pub action_info: EnemyActionInfo,
}

impl EnemyDefinition {
    /// Helper to convert from [`EnemyAsset`] to [`EnemyDefinition`]
    pub(crate) fn from_asset(
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

        // Helper to get an animation index by its name
        let get = |s: &str| {
            named_indices.get(s).ok_or(EnemyDefinitionLoadError::MissingAnimation(s.to_string())).copied()
        };
        // Get an animation index if a name was provided
        let try_get = |s_maybe: &Option<String>| match s_maybe {
            Some(s) => get(s).map(Some),
            None => Ok(None),
        };

        Ok(Self {
            id: asset.id.clone(),
            sprite: asset_server.load(&asset.sprite),
            attributes: asset.attributes.clone(),
            gltf: asset_server.load(&asset.gltf),
            scene: gltf.scenes.first().ok_or(EnemyDefinitionLoadError::NoDefaultScene)?.clone(),
            graph: graphs.add(graph),
            animation_transition_duration_ms: asset.animation_transition_duration_ms,
            movement_animation_info: EnemyMovementAnimationInfo::from_asset(
                &get,
                &try_get,
                &asset.movement_animations,
            )?,
            action_info: EnemyActionInfo::from_asset(&get, &asset.actions)?,
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
    MissingAnimation(String),
}
