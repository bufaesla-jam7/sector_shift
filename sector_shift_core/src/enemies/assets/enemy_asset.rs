use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Asset, TypePath)]
pub struct EnemyAsset {
    pub id: String,
    pub sprite: String,
}

impl EnemyAsset {
    pub const ASSET_PATH: &'static str = "./assets/enemies";
    pub const EXTENSION: &'static str = "enemy";
}
