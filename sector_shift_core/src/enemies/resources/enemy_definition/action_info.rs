use std::{ops::Range, time::Duration};

use bevy::prelude::*;

use crate::enemies::{assets, resources::EnemyDefinitionLoadError};

#[derive(Reflect, Clone)]
pub struct EnemyActionInfo {
    pub primary_attack: EnemyAttack,
    /// Not every enemy needs two attacks...?
    pub secondary_attack: Option<EnemyAttack>,
}

#[derive(Reflect, Clone)]
pub struct EnemyAttack {
    /// Minimum and maximum range of the attack
    pub range: Range<f32>,
    /// How long it takes for the action to take effect, i.e. to actually deal damage for melee
    /// attacks or to fire the projectile for ranged attacks
    pub cast_duration: Duration,
    pub kind: AttackType,
    pub animation_index: AnimationNodeIndex,
    pub animation_playback_speed: f32,
}

#[derive(Reflect, Clone)]
/// Currently identical to [`assets::AttackType`], but probably not anymore once [`Self::Ranged`]
/// is added
pub enum AttackType {
    Melee {
        damage: i32,
        /// Dimensions of a cuboid collider
        hitbox: Vec3,
    },
    Ranged {
        // TODO
    },
}

impl EnemyActionInfo {
    pub(crate) fn from_asset(
        get: &dyn Fn(&str) -> Result<AnimationNodeIndex, EnemyDefinitionLoadError>,
        asset: &assets::EnemyActionInfo,
    ) -> Result<Self, EnemyDefinitionLoadError> {
        Ok(Self {
            primary_attack: EnemyAttack::from_asset(get, &asset.primary_attack)?,
            secondary_attack: match &asset.secondary_attack {
                Some(attack) => Some(EnemyAttack::from_asset(get, attack)?),
                None => None,
            },
        })
    }
}

impl EnemyAttack {
    fn from_asset(
        get: &dyn Fn(&str) -> Result<AnimationNodeIndex, EnemyDefinitionLoadError>,
        asset: &assets::EnemyAttack,
    ) -> Result<Self, EnemyDefinitionLoadError> {
        Ok(Self {
            range: asset.range.clone(),
            cast_duration: Duration::from_millis(asset.cast_duration_ms),
            kind: AttackType::from_asset(&asset.kind),
            animation_index: get(&asset.animation_name)?,
            animation_playback_speed: asset.animation_playback_speed,
        })
    }
}

impl AttackType {
    fn from_asset(asset: &assets::AttackType) -> Self {
        match asset {
            assets::AttackType::Melee { damage, hitbox } => Self::Melee {
                damage: *damage,
                hitbox: *hitbox,
            },
            assets::AttackType::Ranged {} => Self::Ranged {},
        }
    }
}
