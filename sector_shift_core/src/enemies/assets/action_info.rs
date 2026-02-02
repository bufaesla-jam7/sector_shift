use std::ops::Range;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct EnemyActionInfo {
    pub primary_attack: EnemyAttack,
    /// Not every enemy needs two attacks...?
    pub secondary_attack: Option<EnemyAttack>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct EnemyAttack {
    /// Minimum and maximum range of the attack, to be used by the enemy AI
    pub range: Range<f32>,
    /// How long it takes for the action to take effect, i.e. to actually deal damage for melee
    /// attacks or to fire the projectile for ranged attacks
    pub cast_duration_ms: u64,
    pub kind: AttackType,
    pub animation_name: String,
    pub animation_playback_speed: f32,
}

#[derive(Serialize, Deserialize)]
pub(crate) enum AttackType {
    Melee {
        damage: i32,
        /// Dimensions of a cuboid collider
        hitbox: Vec3,
    },
    Ranged {
        // TODO
    },
}
