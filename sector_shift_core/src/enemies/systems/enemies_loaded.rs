use bevy::prelude::*;

use crate::enemies::resources::EnemyLibrary;

/// This function is used as a SystemCondition to wait for all enemies to be loaded during the asset loading state.
pub fn enemies_loaded(enemy_library: Res<EnemyLibrary>) -> bool {
    enemy_library.is_fully_loaded()
}
