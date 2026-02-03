use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    enemies::assets::{EnemyActionInfo, EnemyMovementAnimationNames},
    prelude::Enemy,
};

/// This gets loaded by the AssetManager
/// Represents an enemy type that can be spawned in levels
/// Need to add more data later for AI, stats, etc.
#[derive(Serialize, Deserialize, Asset, TypePath)]
pub(crate) struct EnemyAsset {
    /// This must be unique as it is used as the key when looking up from the [`crate::prelude::EnemyLibrary`] resource or MapObject::Enemy
    pub id: String,
    /// This is the path to the image to be loaded
    pub sprite: String,
    /// This is the path to the gltf model to be loaded
    pub gltf: String,
    #[serde(skip)]
    #[dependency]
    /// Asset dependency, the gltf has to be loaded before we can convert [`EnemyAsset`] to
    /// [`crate::enemies::resources::EnemyDefinition`]
    pub gltf_handle: Option<Handle<Gltf>>,
    /// Attributes directly defining [`Enemy`] component
    pub attributes: Enemy,
    /// Duration a transition between two animations takes, in milliseconds
    pub animation_transition_duration_ms: u64,
    /// Names of movement animations to be extracted from the gltf
    /// Needed to build the [`crate::enemies::resources::EnemyMovementAnimationInfo`] component
    pub movement_animations: EnemyMovementAnimationNames,
    /// The definition of the attacks an enemy can perform
    pub actions: EnemyActionInfo,
}

impl EnemyAsset {
    /// The file extension for these assets
    pub const EXTENSION: &'static str = "enemy";
}
