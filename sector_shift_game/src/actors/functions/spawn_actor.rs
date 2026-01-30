use avian3d::prelude::*;
use bevy::prelude::*;

use crate::actors::components::Health;

pub fn spawn_actor(commands: &mut Commands, transform: Transform, health: i32) -> Entity {
    commands
        .spawn((
            RigidBody::Dynamic,
            Collider::capsule(0.5, 1.0),
            LockedAxes::ROTATION_LOCKED,
            Restitution::new(0.0),
            Friction::new(1.0),
            LinearVelocity::default(),
            transform,
            Health::new(health),
        ))
        .id()
}
