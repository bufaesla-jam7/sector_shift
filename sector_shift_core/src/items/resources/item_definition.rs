use bevy::prelude::*;

use crate::items::{assets::ItemAsset, data::ItemEffect};

pub struct ItemDefinition {
    pub id: String,
    pub effect: ItemEffect,
    pub sprite: Handle<Image>,
}

impl ItemDefinition {
    pub fn from_asset(asset_server: &AssetServer, asset: &ItemAsset) -> Self {
        Self {
            id: asset.id.clone(),
            effect: asset.effect.clone(),
            sprite: asset_server.load(&asset.sprite),
        }
    }
}
