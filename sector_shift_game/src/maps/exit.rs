use avian3d::prelude::{Collider, CollisionEventsEnabled, CollisionStart, Sensor};
use bevy::prelude::*;

use crate::{actors::components::Player, maps::functions::SpawnLevel};

/// spawns an exit at the given transform with
/// an Exit component
/// Collider component
/// Observer
pub fn spawn_exit(commands: &mut Commands, level_id: &str, transform: Transform) -> Option<Entity> {
    let entity = commands
        .spawn((
            transform,
            Exit {
                level_id: level_id.to_string(),
            },
            Sensor,
            Collider::capsule(0.5, 1.0),
            CollisionEventsEnabled,
        ))
        .observe(on_player_touches_exit)
        .id();

    Some(entity)
}

fn on_player_touches_exit(
    event: On<CollisionStart>,
    mut spawn_level: MessageWriter<SpawnLevel>,
    exit: Query<&Exit>,
    player: Single<Entity, With<Player>>,
) {
    let exit_entity = event.collider1;
    let other_entity = event.collider2;

    if *player == other_entity {
        debug!("{other_entity} touched exit {exit_entity}");

        /* Call spawn level */
        let exit = exit.get(exit_entity).unwrap();
        spawn_level.write(SpawnLevel {
            level_id: exit.level_id.clone(),
        });
    }
}

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub struct Exit {
    level_id: String,
}
