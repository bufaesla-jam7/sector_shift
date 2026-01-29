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

impl MapObject {
    /// Returns the color associated with the object type for editor visualization.
    pub fn color(&self) -> Color {
        match self {
            MapObject::Exit(_) => Color::srgb(0.0, 0.0, 1.0),
            MapObject::Enemy(_) => Color::srgb(1.0, 0.0, 0.0),
            MapObject::Item(_) => Color::srgb(0.0, 1.0, 0.0),
        }
    }
}
