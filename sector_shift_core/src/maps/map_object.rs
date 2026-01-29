use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents any object that can be placed on the map.
#[derive(Serialize, Deserialize, Reflect, Debug, Clone)]
pub enum MapObject {
    /// An exit leading to another level.
    Exit(String),
    /// An enemy with a specific ID.
    Enemy(String),
    /// An item with a specific ID.
    Item(String),
}
