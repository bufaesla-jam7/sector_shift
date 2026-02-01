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

#[derive(Component, Clone, Default)]
/// Component storing the [`AnimationNodeIndex`] referencing different movement animations.
/// It is stored on the same entity as the [`Enemy`] component
pub struct EnemyMovementAnimationInfo {
    /// Index of the idle animation
    pub idle: AnimationNodeIndex,
    pub idle_playback_speed: f32,
    /// Index of the walking animation
    pub walk_forwards: AnimationNodeIndex,
    pub forward_playback_speed: f32,
    /// Index of the backwards walking animation.
    /// Will fallback to playing the forward animation in reverse
    pub walk_backwards: Option<AnimationNodeIndex>,
    pub backward_playback_speed: f32,
    /// Index of the walking left animation
    /// Will try to fallback to playing the walking right animation in reverse,
    /// else the idle animation will be used
    pub walk_left: Option<AnimationNodeIndex>,
    pub left_playback_speed: f32,
    /// Index of the walking right animation
    /// Will try to fallback to playing the walking left animation in reverse,
    /// else the idle animation will be used
    pub walk_right: Option<AnimationNodeIndex>,
    pub right_playback_speed: f32,
}
