use crate::prelude::*;
use bevy::{asset::UnapprovedPathMode, prelude::*};

pub fn all_assets_loaded(
    enemy_library: Res<EnemyLibrary>,
    item_library: Res<ItemLibrary>,
    env_library: Res<EnvObjLibrary>,
) -> bool {
    enemy_library.is_ready() && item_library.is_ready() && env_library.is_ready()
}

pub fn asset_plugin_with_fixed_path() -> AssetPlugin {
    let asset_path = env!("CARGO_MANIFEST_DIR").replacen("sector_shift_core", "assets", 1);

    AssetPlugin {
        file_path: asset_path,
        // Allow scenes to be loaded from anywhere on disk
        unapproved_path_mode: UnapprovedPathMode::Allow,
        ..default()
    }
}
