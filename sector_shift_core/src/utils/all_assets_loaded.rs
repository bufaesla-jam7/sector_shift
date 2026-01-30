use crate::{enemies::systems::enemies_loaded, prelude::*};
use bevy::prelude::*;

pub fn all_assets_loaded(
    enemy_library: Res<EnemyLibrary>,
    item_library: Res<ItemLibrary>,
    env_library: Res<EnvObjLibrary>,
) -> bool {
    enemies_loaded(enemy_library) && item_library.is_ready() && env_library.is_ready()
}
