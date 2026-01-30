use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::items::data::ItemEffect;

/// This gets loaded by the AssetManager
/// Represents an item type that can be picked up in levels
#[derive(Serialize, Deserialize, Asset, TypePath)]
pub struct ItemAsset {
    /// This must be unique as it is used as the key when looking up from the "ItemLibrary" resource or MapObject::Item
    pub id: String,
    /// The effect this item has when picked up
    pub effect: ItemEffect,
    /// This is the path to the image to be loaded
    pub sprite: String,
}

impl ItemAsset {
    /// The file extension for these assets
    pub const EXTENSION: &'static str = "item";
}
