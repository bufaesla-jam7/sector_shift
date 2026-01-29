use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::{MOUSE_SENSITIVITY, actors::components::Player};

pub fn player_look(
    mut mouse_motion: MessageReader<MouseMotion>,
    mut player_transform: Single<&mut Transform, With<Player>>,
    mut camera_transform: Single<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    let mut rotation_delta = Vec2::ZERO;

    for event in mouse_motion.read() {
        rotation_delta += event.delta;
    }

    if rotation_delta == Vec2::ZERO {
        return;
    }

    player_transform.rotate_y(-rotation_delta.x * MOUSE_SENSITIVITY);

    let (yaw, mut pitch, roll) = camera_transform.rotation.to_euler(EulerRot::YXZ);
    pitch -= rotation_delta.y * MOUSE_SENSITIVITY;
    pitch = pitch.clamp(-1.54, 1.54);

    camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
}
