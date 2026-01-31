use std::f32::consts::TAU;

use bevy::prelude::*;
use sector_shift_core::environment::components::Skybox;

pub fn rotate_skybox(time: Res<Time>, skybox: Query<(&mut Transform, &Skybox)>) {
    for (mut transform, skybox) in skybox {
        transform.rotate(Quat::from_axis_angle(
            skybox.rotation_axis,
            time.delta_secs() * skybox.rotation_speed / TAU,
        ));
    }
}
