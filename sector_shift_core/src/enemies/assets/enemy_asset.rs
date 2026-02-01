use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::Enemy;

/// This gets loaded by the AssetManager
/// Represents an enemy type that can be spawned in levels
/// Need to add more data later for AI, stats, etc.
#[derive(Serialize, Deserialize, Asset, TypePath)]
pub struct EnemyAsset {
    /// This must be unique as it is used as the key when looking up from the [`EnemyLibrary`] resource or MapObject::Enemy
    pub id: String,
    /// This is the path to the image to be loaded
    pub sprite: String,
    /// This is the path to the gltf model to be loaded
    pub gltf: String,
    #[serde(skip)]
    #[dependency]
    /// Asset dependency, the gltf has to be loaded before we can convert [`EnemyAsset`] to
    /// [`EnemyDefinition`]
    pub gltf_handle: Option<Handle<Gltf>>,
    /// Attributes directly defining [`Enemy`] component
    pub attributes: Enemy,
    /// Duration a transition between two animations takes, in milliseconds
    pub animation_transition_duration_ms: u64,
    /// Names of movement animations to be extracted from the gltf
    /// Needed to build the [`EnemyMovementAnimationInfo`] component
    pub movement_animations: EnemyMovementAnimationNames,
}

impl EnemyAsset {
    /// The file extension for these assets
    pub const EXTENSION: &'static str = "enemy";
}

#[derive(Serialize, Deserialize)]
/// see [`EnemyMovementAnimationInfo`]
pub struct EnemyMovementAnimationNames {
    pub idle: String,

    #[serde(default = "one")]
    pub idle_playback_speed: f32,

    pub walk_forwards: String,

    #[serde(default = "one")]
    pub forward_playback_speed: f32,

    #[serde(default)]
    pub walk_backwards: Option<String>,

    #[serde(default = "one")]
    pub backward_playback_speed: f32,

    #[serde(default)]
    pub walk_left: Option<String>,

    #[serde(default = "one")]
    pub left_playback_speed: f32,

    #[serde(default)]
    pub walk_right: Option<String>,

    #[serde(default = "one")]
    pub right_playback_speed: f32,
}

/// The serde derive does not allow providing default values inline, they have to be set via
/// [`Default::default()`] or a function.
fn one() -> f32 {
    1.
}
