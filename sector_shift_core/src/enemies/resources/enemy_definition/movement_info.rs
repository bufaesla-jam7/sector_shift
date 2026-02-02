use bevy::prelude::*;

use crate::enemies::{assets::EnemyMovementAnimationNames, resources::EnemyDefinitionLoadError};

#[derive(Clone, Default)]
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

impl EnemyMovementAnimationInfo {
    pub(crate) fn from_asset(
        get: &dyn Fn(&str) -> Result<AnimationNodeIndex, EnemyDefinitionLoadError>,
        try_get: &dyn Fn(&Option<String>) -> Result<Option<AnimationNodeIndex>, EnemyDefinitionLoadError>,
        asset: &EnemyMovementAnimationNames,
    ) -> Result<Self, EnemyDefinitionLoadError> {
        Ok(Self {
            idle: get(&asset.idle)?,
            idle_playback_speed: asset.idle_playback_speed,
            walk_forwards: get(&asset.walk_forwards)?,
            forward_playback_speed: asset.forward_playback_speed,
            walk_backwards: try_get(&asset.walk_backwards)?,
            backward_playback_speed: asset.backward_playback_speed,
            walk_left: try_get(&asset.walk_left)?,
            left_playback_speed: asset.left_playback_speed,
            walk_right: try_get(&asset.walk_right)?,
            right_playback_speed: asset.right_playback_speed,
        })
    }
}
