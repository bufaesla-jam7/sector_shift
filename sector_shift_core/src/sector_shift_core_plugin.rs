use bevy::prelude::*;

use crate::{enemies::EnemiesPlugin, items::ItemsPlugin, utils::UtilsPlugin};

pub struct SectorShiftCorePlugin<T: States + Copy> {
    enemies_plugin: EnemiesPlugin<T>,
    items_plugin: ItemsPlugin<T>,
}

impl<T: States + Copy> SectorShiftCorePlugin<T> {
    pub fn new(asset_load_state: T) -> Self {
        Self {
            enemies_plugin: EnemiesPlugin::new(asset_load_state),
            items_plugin: ItemsPlugin::new(asset_load_state),
        }
    }
}

impl<T: States + Copy> Plugin for SectorShiftCorePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugins(self.enemies_plugin.clone());
        app.add_plugins(self.items_plugin.clone());

        app.add_plugins(UtilsPlugin);
    }
}
