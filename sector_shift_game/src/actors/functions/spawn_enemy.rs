use avian3d::prelude::*;
use bevy::prelude::*;
use sector_shift_core::prelude::*;

use crate::actors::enemy_controller::EnemyController;

/// Spawns an enemy with the following components:
/// - [`Name`]
/// - [`Enemy`]
/// - [`EnemyController`]
/// - [`RigidBody`] and [`LockedAxes`] as required by [`EnemyController`]
/// - [`SceneRoot`]
/// - [`Transform`]
/// - [`Collider`] (capsule)
///
pub fn spawn_enemy(
    commands: &mut Commands,
    enemy_library: &EnemyLibrary,
    enemy_id: &str,
    transform: Transform,
) -> Option<Entity> {
    if let Some(definition) = enemy_library.get(enemy_id) {
        let entity = commands
            .spawn((
                Name::new(definition.id.clone()),
                definition.attributes.clone(),
                EnemyController::default(),
                SceneRoot(definition.scene.clone()),
                transform,
                Collider::capsule(0.5, 1.0), // Match sprite size
            ))
            .id();
        Some(entity)
    } else {
        warn!("Enemy ID '{}' not found in EnemyLibrary.", enemy_id);
        None
    }
}
