use bevy::prelude::*;

use crate::CELL_SIZE;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        // TODO: Do not hard code editor map size
        Transform::from_xyz(32.0 / 2.0 * CELL_SIZE, 32.0 / 2.0 * CELL_SIZE, 0.0),
    ));
}
