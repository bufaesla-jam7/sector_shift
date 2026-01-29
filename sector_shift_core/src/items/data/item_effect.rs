use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// The effect an item has when picked up
#[derive(Serialize, Deserialize, Reflect, Debug, Clone)]
pub enum ItemEffect {
    /// Restore a certain amount of health
    /// (Amount)
    Health(i32),
    /// Add ammo for a specific weapon
    /// (WeaponId, Amount)
    Ammo(String, u32),
    /// Give the player a new weapon
    /// (WeaponId)
    Weapon(String),
    /// Increase the player's score
    /// (Amount)
    Score(u32),
}

impl Default for ItemEffect {
    fn default() -> Self {
        ItemEffect::Score(100)
    }
}
