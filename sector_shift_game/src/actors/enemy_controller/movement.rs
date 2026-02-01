use std::f32::consts::TAU;

use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;
use sector_shift_core::prelude::Enemy;

use crate::actors::enemy_controller::{EnemyController, MovementState};

pub fn apply_rotation(time: Res<Time>, query: Query<(&Enemy, &EnemyController, &mut Transform)>) {
    for (enemy, controller, mut transform) in query {
        if transform.rotation != controller.rotation {
            transform.rotation = transform.rotation.rotate_towards(
                controller.rotation,
                time.delta_secs() * enemy.rotation_speed * TAU,
            );
        }
    }
}

pub fn apply_velocity(query: Query<(&Enemy, &EnemyController, &mut LinearVelocity)>) {
    for (enemy, controller, mut velocity) in query {
        if controller.movement_state != MovementState::Idle && !controller.is_acting() {
            let direction = match controller.movement_state {
                MovementState::Idle => unreachable!(),
                MovementState::Forward => Vec3::Z,
                MovementState::Backward => Vec3::NEG_Z,
                MovementState::Left => Vec3::X,
                MovementState::Right => Vec3::NEG_X,
            };
            velocity.0 = controller.rotation * direction * enemy.movement_velocity;
        } else if velocity.length() != 0. {
            velocity.0 = Vec3::ZERO;
        }
    }
}
