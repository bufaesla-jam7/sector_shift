use bevy::prelude::*;

/// Enemy component
/// This should hold AI data
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Enemy;
