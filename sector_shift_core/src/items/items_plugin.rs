use bevy::prelude::*;

use crate::{
    items::{
        assets::ItemAsset,
        resources::ItemLibrary,
        systems::{load_items, process_items},
    },
    utils::generic_asset_loader::GenericAssetLoader,
};

#[derive(Clone)]
pub struct ItemsPlugin<T: States + Copy> {
    asset_load_state: T,
}

impl<T: States + Copy> ItemsPlugin<T> {
    pub fn new(asset_load_state: T) -> Self {
        Self { asset_load_state }
    }
}

impl<T: States + Copy> Plugin for ItemsPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_asset::<ItemAsset>();
        app.register_asset_loader(GenericAssetLoader::<ItemAsset>::new(&[
            ItemAsset::EXTENSION,
        ]));

        app.init_resource::<ItemLibrary>();

        app.add_systems(OnEnter(self.asset_load_state), load_items);
        app.add_systems(
            Update,
            process_items.run_if(in_state(self.asset_load_state)),
        );
    }
}
