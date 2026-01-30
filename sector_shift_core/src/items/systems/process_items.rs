use bevy::prelude::*;

use crate::items::{
    assets::ItemAsset,
    resources::{ItemDefinition, ItemLibrary},
};

/// A system that processes loaded item assets and adds them to the ItemLibrary
pub fn process_items(
    asset_server: Res<AssetServer>,
    a_items: Res<Assets<ItemAsset>>,
    mut item_library: ResMut<ItemLibrary>,
) {
    if item_library.loading_finished {
        for handle in std::mem::take(&mut item_library.loading) {
            if let Some(item_asset) = a_items.get(&handle) {
                info!("Loaded item asset with id: {}", item_asset.id);
                item_library.add(ItemDefinition::from_asset(&asset_server, item_asset));
            }
        }
    }
}
