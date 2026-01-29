use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::maps::DoorAxis;

/// Represents the different types of tiles in the game.
#[derive(Serialize, Deserialize, Reflect, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TileType {
    /// A solid wall tile.
    #[default]
    Wall,
    /// A door leading to another area.
    Door(DoorAxis),
    /// A walkable floor tile.
    Floor,
}

impl TileType {
    /// Returns the color associated with the tile type for editor visualization.
    pub fn color(self) -> Color {
        match self {
            TileType::Wall => Color::srgb(0.8, 0.8, 0.8),
            TileType::Floor => Color::srgb(0.0, 0.0, 0.0),
            TileType::Door(_axis) => Color::srgb(0.5, 0.5, 0.5),
        }
    }
}
