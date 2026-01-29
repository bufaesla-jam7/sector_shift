use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// This gets loaded by the AssetManager
/// Represents an enemy type that can be spawned in levels
/// Need to add more data later for AI, stats, etc.
#[derive(Serialize, Deserialize, Asset, TypePath)]
pub struct EnemyAsset {
    /// This must be unique as it is used as the key when looking up from the "EnemyLibrary" resource or MapObject::Enemy
    pub id: String,
    /// This is the path to the image to be loaded
    pub sprite: String,
}

impl EnemyAsset {
    /// Where these assets are stored
    pub const ASSET_PATH: &'static str = "../assets/enemies";
    /// The file extension for these assets
    pub const EXTENSION: &'static str = "enemy";
}
