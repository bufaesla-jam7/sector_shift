use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(1024.0, 1024.0, 0.0)));
}
