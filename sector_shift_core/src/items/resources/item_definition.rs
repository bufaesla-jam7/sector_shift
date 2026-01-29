use bevy::prelude::*;

use crate::items::{assets::ItemAsset, data::ItemEffect};

/// An intermediate step between an item asset and a spawned item
pub struct ItemDefinition {
    /// The unique ID of the item used by MapObject::Item
    pub id: String,
    /// The effect this item has when picked up
    pub effect: ItemEffect,
    /// The sprite handle for the item
    pub sprite: Handle<Image>,
}

impl ItemDefinition {
    /// Helper to convert from ItemAsset to ItemDefinition
    pub fn from_asset(asset_server: &AssetServer, asset: &ItemAsset) -> Self {
        Self {
            id: asset.id.clone(),
            effect: asset.effect.clone(),
            sprite: asset_server.load(&asset.sprite),
        }
    }
}
