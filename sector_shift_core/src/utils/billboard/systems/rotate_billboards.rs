use bevy::prelude::*;

use crate::utils::billboard::components::Billboard;

pub fn rotate_billboards(
    q_camera: Single<&GlobalTransform, With<Camera>>,
    mut q_billboards: Query<&mut Transform, With<Billboard>>,
) {
    let camera_transform = *q_camera;

    for mut transform in q_billboards.iter_mut() {
        // TOOD: Should probably be xz plane only
        transform.look_at(camera_transform.translation(), Vec3::Y);
    }
}
