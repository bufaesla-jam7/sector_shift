use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{PLAYER_SPEED, actors::components::Player};

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    q_player: Single<(&Transform, &mut LinearVelocity), With<Player>>,
) {
    let (player_transform, mut linear_velocity) = q_player.into_inner();
    let mut direction = Vec3::ZERO;

    // Walk
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction -= player_transform.local_z() * 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction += player_transform.local_z() * 1.0;
    }

    // Strafe
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction -= player_transform.local_x() * 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction += player_transform.local_x() * 1.0;
    }

    // Flatten to the XZ plane
    direction.y = 0.0;

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
        linear_velocity.x = direction.x * PLAYER_SPEED;
        linear_velocity.z = direction.z * PLAYER_SPEED;
    } else {
        // Stop moving
        linear_velocity.x = 0.0;
        linear_velocity.z = 0.0;
    }
}
