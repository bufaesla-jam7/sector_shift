use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::maps::DoorAxis;

#[derive(Serialize, Deserialize, Reflect, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TileType {
    #[default]
    Wall,
    Door(DoorAxis),
    Floor,
}

impl TileType {
    pub fn color(self) -> Color {
        match self {
            TileType::Wall => Color::srgb(0.8, 0.8, 0.8),
            TileType::Floor => Color::srgb(0.0, 0.0, 0.0),
            TileType::Door(_axis) => Color::srgb(0.5, 0.5, 0.5),
        }
    }
}
