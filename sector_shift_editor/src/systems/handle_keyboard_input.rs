use bevy::prelude::*;

const SPEED: f32 = 10.0;

pub fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_camera: Single<&mut Transform, With<Camera>>,
) {
    let mut transform = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) {
        transform.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        transform.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        transform.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        transform.x += 1.0;
    }

    if transform != Vec2::ZERO {
        q_camera.translation.x += transform.x * SPEED;
        q_camera.translation.y += transform.y * SPEED;
    }
}
