use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    items::{components::Item, resources::ItemLibrary},
    utils::billboard::components::Billboard,
};

/// Spawns an item with the following components:
/// - Name
/// - Item
/// - Sprite
/// - Transform
/// - Billboard
/// - Collider (cylinder)
/// - Sensor
///
pub fn spawn_item(
    commands: &mut Commands,
    item_library: &ItemLibrary,
    item_id: &str,
    transform: Transform,
) -> Option<Entity> {
    if let Some(item_definition) = item_library.get(item_id) {
        let entity = commands
            .spawn((
                Name::new(item_definition.id.clone()),
                Item {
                    effect: item_definition.effect.clone(),
                },
                Sprite {
                    image: item_definition.sprite.clone(),
                    custom_size: Some(Vec2::new(1.0, 1.0)), // Match collider
                    ..Default::default()
                },
                transform,
                Billboard,
                Collider::cylinder(0.5, 1.0), // Match sprite size
                Sensor,
            ))
            .id();
        Some(entity)
    } else {
        warn!("Item ID '{}' not found in ItemLibrary.", item_id);
        None
    }
}
