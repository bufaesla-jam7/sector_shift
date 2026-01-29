use bevy::prelude::*;
use sector_shift_core::prelude::*;

#[derive(Resource)]
pub struct MapData {
    pub level: Level,
}

impl Default for MapData {
    fn default() -> Self {
        Self {
            level: Level::new("Level1", (64, 64)),
        }
    }
}
