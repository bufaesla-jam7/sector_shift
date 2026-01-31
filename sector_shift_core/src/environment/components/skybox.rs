use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Skybox {
    /// measured in degree per second
    pub rotation_speed: f32,
    pub rotation_axis: Vec3,
}
