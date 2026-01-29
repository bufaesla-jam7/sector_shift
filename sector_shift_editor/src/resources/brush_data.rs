use bevy::prelude::*;
use sector_shift_core::prelude::TileType;

use crate::data::BrushType;

#[derive(Resource)]
pub struct BrushData {
    pub brush: BrushType,
}

impl Default for BrushData {
    fn default() -> Self {
        BrushData {
            brush: BrushType::Tile(TileType::Floor),
        }
    }
}
