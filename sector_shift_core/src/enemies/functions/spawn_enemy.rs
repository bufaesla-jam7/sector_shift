use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    enemies::{components::Enemy, resources::EnemyLibrary},
    utils::billboard::components::Billboard,
};

/// Spawns an enemy with the following components:
/// - Name
/// - Enemy
/// - Sprite
/// - Transform
/// - Billboard
/// - Collider (capsule)
///
pub fn spawn_enemy(
    commands: &mut Commands,
    enemy_library: &EnemyLibrary,
    enemy_id: &str,
    transform: Transform,
) -> Option<Entity> {
    if let Some(enemy_definition) = enemy_library.get(enemy_id) {
        let entity = commands
            .spawn((
                Name::new(enemy_definition.id.clone()),
                Enemy,
                Sprite {
                    image: enemy_definition.sprite.clone(),
                    custom_size: Some(Vec2::new(1.0, 2.0)), // Match collider
                    ..Default::default()
                },
                transform,
                Billboard,
                Collider::capsule(0.5, 1.0), // Match sprite size
            ))
            .id();
        Some(entity)
    } else {
        warn!("Enemy ID '{}' not found in EnemyLibrary.", enemy_id);
        None
    }
}
