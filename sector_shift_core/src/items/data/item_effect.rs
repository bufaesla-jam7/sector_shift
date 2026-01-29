use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Reflect, Debug, Clone)]
pub enum ItemEffect {
    Health(i32),
    Ammo(String, u32),
    Weapon(String),
    Score(u32),
}

impl Default for ItemEffect {
    fn default() -> Self {
        ItemEffect::Score(100)
    }
}
