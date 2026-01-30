use crate::prelude::*;
use bevy::prelude::*;

pub fn all_assets_loaded(
    enemy_library: Res<EnemyLibrary>,
    item_library: Res<ItemLibrary>,
    env_library: Res<EnvObjLibrary>,
) -> bool {
    enemy_library.is_ready() && item_library.is_ready() && env_library.is_ready()
}
