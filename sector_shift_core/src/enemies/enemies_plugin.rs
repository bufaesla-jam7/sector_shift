use bevy::prelude::*;

use crate::{
    enemies::{
        assets::EnemyAsset,
        resources::EnemyLibrary,
        systems::{load_enemies, process_enemies},
    },
    utils::generic_asset_loader::GenericAssetLoader,
};

#[derive(Clone)]
pub struct EnemiesPlugin<T: States + Copy> {
    asset_load_state: T,
}

impl<T: States + Copy> EnemiesPlugin<T> {
    pub fn new(asset_load_state: T) -> Self {
        Self { asset_load_state }
    }
}

impl<T: States + Copy> Plugin for EnemiesPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_asset::<EnemyAsset>();
        app.register_asset_loader(GenericAssetLoader::<EnemyAsset>::new(&[
            EnemyAsset::EXTENSION,
        ]));

        app.init_resource::<EnemyLibrary>();

        app.add_systems(OnEnter(self.asset_load_state), load_enemies);
        app.add_systems(
            Update,
            process_enemies.run_if(in_state(self.asset_load_state)),
        );
    }
}
