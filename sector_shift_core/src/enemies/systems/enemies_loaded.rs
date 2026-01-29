use bevy::prelude::*;

use crate::enemies::resources::EnemyLibrary;

pub fn enemies_loaded(enemy_library: Res<EnemyLibrary>) -> bool {
    enemy_library.loading.is_empty()
}
