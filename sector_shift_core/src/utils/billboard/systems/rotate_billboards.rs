use bevy::prelude::*;

use crate::utils::billboard::components::Billboard;

pub fn rotate_billboards(
    q_camera: Single<&Transform, (With<Camera>, Without<Billboard>)>,
    mut q_billboards: Query<&mut Transform, With<Billboard>>,
) {
    let camera_transform = *q_camera;

    for mut transform in q_billboards.iter_mut() {
        transform.look_at(camera_transform.translation, Vec3::Y);
        transform.rotation = Quat::from_rotation_y(transform.rotation.to_euler(EulerRot::YXZ).0);
    }
}
