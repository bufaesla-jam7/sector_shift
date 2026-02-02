use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Enemy component
/// This should hold AI data
#[derive(Component, Reflect, Default, Serialize, Deserialize, Clone)]
#[reflect(Component)]
pub struct Enemy {
    /// Measured in full rotations (360°) per second
    pub rotation_speed: f32,
    /// Measured in meters per second
    pub movement_velocity: f32,
}
