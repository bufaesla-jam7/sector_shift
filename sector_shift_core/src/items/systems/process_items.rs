use bevy::prelude::*;

use crate::items::{
    assets::ItemAsset,
    resources::{ItemDefinition, ItemLibrary},
};

pub fn process_items(
    mut e_item_asset: MessageReader<AssetEvent<ItemAsset>>,
    asset_server: Res<AssetServer>,
    a_items: Res<Assets<ItemAsset>>,
    mut item_library: ResMut<ItemLibrary>,
) {
    for event in e_item_asset.read() {
        if let AssetEvent::LoadedWithDependencies { id } = event {
            let index = item_library.loading.iter().position(|handle| handle.id() == *id);
            if let Some(index) = index {
                if let Some(item_asset) = a_items.get(*id) {
                    item_library.add(ItemDefinition::from_asset(&asset_server, item_asset));
                    info!("Loaded item asset");
                }
                item_library.loading.swap_remove(index);
            }
        }
    }
}
