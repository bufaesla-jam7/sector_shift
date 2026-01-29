use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::items::data::ItemEffect;

#[derive(Serialize, Deserialize, Asset, TypePath)]
pub struct ItemAsset {
    pub id: String,
    pub effect: ItemEffect,
    pub sprite: String,
}

impl ItemAsset {
    pub const ASSET_PATH: &'static str = "./assets/items";
    pub const EXTENSION: &'static str = "item";
}
