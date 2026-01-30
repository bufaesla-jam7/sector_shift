use avian3d::prelude::*;
use bevy::prelude::*;
use sector_shift_core::prelude::*;

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
    item_mesh: Handle<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Option<Entity> {
    if let Some(item_definition) = item_library.get(item_id) {
        // TODO: Move this out so it is only created once?
        let material = materials.add(StandardMaterial {
            base_color_texture: Some(item_definition.sprite.clone()),
            cull_mode: None, // TODO: Rectangle has texture on -z? Is there a better way?
            ..Default::default()
        });
        let entity = commands
            .spawn((
                Name::new(item_definition.id.clone()),
                Item {
                    effect: item_definition.effect.clone(),
                },
                Mesh3d(item_mesh),
                MeshMaterial3d(material),
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
