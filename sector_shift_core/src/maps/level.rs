use std::path::PathBuf;

use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    maps::{MapObject, TileType},
    utils::{direction::Direction, grid::Grid},
};

#[derive(Debug, Error)]
pub enum LevelError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("RON error: {0}")]
    Ron(#[from] ron::error::Error),
    #[error("RON spanned error: {0}")]
    RonSpanned(#[from] ron::error::SpannedError),
}

/// A Level represents a game level with its tiles, player start position, and objects.
#[derive(Serialize, Deserialize, Asset, TypePath)]
pub struct Level {
    /// The unique identifier for the level. Used in MapObject::Exit.
    pub id: String,
    /// These are the "blocks" that make up the level.
    pub tiles: Grid<TileType>,
    /// The starting position and direction of the player.
    pub player_start: ((i32, i32), Direction),
    /// Map objects placed in the level, keyed by their (x, y) position.
    pub objects: HashMap<(i32, i32), MapObject>,
}

impl Default for Level {
    fn default() -> Self {
        Self::new("Level_01", (32, 32))
    }
}

// Constructor + Serialization methods
impl Level {
    fn get_path_for_id(id: &str) -> PathBuf {
        format!("./assets/levels/{}.ron", id).into()
    }

    pub fn load(id: impl ToString) -> Self {
        Self::try_load(id).unwrap_or_default()
    }

    pub fn try_load(id: impl ToString) -> Result<Self, LevelError> {
        let data = std::fs::read_to_string(Self::get_path_for_id(&id.to_string()))?;
        let level: Self = ron::de::from_str(&data)?;
        Ok(level)
    }

    pub fn save(&self) -> Result<(), LevelError> {
        let data = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())?;
        let path = Self::get_path_for_id(&self.id);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, data)?;
        Ok(())
    }

    pub fn new(id: impl ToString, size: (u32, u32)) -> Self {
        Self {
            id: id.to_string(),
            tiles: Grid::new_default(size),
            player_start: ((0, 0), Direction::NORTH),
            objects: HashMap::new(),
        }
    }
}

// Accessor methods
impl Level {
    pub fn width(&self) -> u32 {
        self.tiles.width()
    }

    pub fn height(&self) -> u32 {
        self.tiles.height()
    }
}

// Level creation
impl Level {
    /// Sets the tile at the given position. Returns true if successful, false if out of bounds.
    pub fn set_tile(&mut self, position: (i32, i32), tile_type: TileType) -> bool {
        if let Some(tile) = self.tiles.get_mut(position) {
            *tile = tile_type;
            true
        } else {
            false
        }
    }

    /// Sets the player start position and direction.
    pub fn set_player_start(&mut self, position: (i32, i32), direction: Direction) {
        // Ensure the player start is on a Floor tile
        if self.set_tile(position, TileType::Floor) {
            self.player_start = (position, direction);
        }
    }

    /// Removes any object at the given position.
    pub fn remove_object(&mut self, position: (i32, i32)) {
        self.objects.remove(&position);
    }

    /// Adds an exit at the given position leading to the specified level.
    pub fn add_exit(&mut self, position: (i32, i32), level_name: impl ToString) {
        // All objects go on Floor tiles
        if self.set_tile(position, TileType::Floor) {
            self.objects.insert(position, MapObject::Exit(level_name.to_string()));
        }
    }

    /// Adds an enemy at the given position with the specified enemy ID.
    pub fn add_enemy(&mut self, position: (i32, i32), enemy_id: impl ToString) {
        // All objects go on Floor tiles
        if self.set_tile(position, TileType::Floor) {
            self.objects.insert(position, MapObject::Enemy(enemy_id.to_string()));
        }
    }

    /// Adds an item at the given position with the specified item ID.
    pub fn add_item(&mut self, position: (i32, i32), item_id: impl ToString) {
        // All objects go on Floor tiles
        if self.set_tile(position, TileType::Floor) {
            self.objects.insert(position, MapObject::Item(item_id.to_string()));
        }
    }
}
