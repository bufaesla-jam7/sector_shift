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

#[derive(Serialize, Deserialize, Asset, TypePath)]
pub struct Level {
    pub name: String,
    pub tiles: Grid<TileType>,
    pub player_start: ((i32, i32), Direction),
    pub objects: HashMap<(i32, i32), Vec<MapObject>>,
}

impl Default for Level {
    fn default() -> Self {
        Self::new("Level_01", (32, 32))
    }
}

impl Level {
    fn get_path_from_name(level_name: &str) -> PathBuf {
        format!("./assets/levels/{}.ron", level_name).into()
    }

    pub fn load(level_name: String) -> Self {
        Self::try_load(level_name).unwrap_or_default()
    }

    pub fn try_load(level_name: String) -> Result<Self, LevelError> {
        let data = std::fs::read_to_string(Self::get_path_from_name(&level_name))?;
        let level: Self = ron::de::from_str(&data)?;
        Ok(level)
    }

    pub fn save(&self) -> Result<(), LevelError> {
        let data = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())?;
        let path = Self::get_path_from_name(&self.name);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, data)?;
        Ok(())
    }

    pub fn new(name: impl ToString, size: (u32, u32)) -> Self {
        Self {
            name: name.to_string(),
            tiles: Grid::new_default(size),
            player_start: ((0, 0), Direction::NORTH),
            objects: HashMap::new(),
        }
    }

    pub fn width(&self) -> u32 {
        self.tiles.width()
    }

    pub fn height(&self) -> u32 {
        self.tiles.height()
    }

    pub fn set_player_start(&mut self, position: (i32, i32), direction: Direction) {
        self.player_start = (position, direction);
    }

    pub fn add_exit(&mut self, position: (i32, i32), level_name: impl ToString) {
        // add or replace any MapObject::Exit at the position
        if let Some(map_object) = self
            .objects
            .get_mut(&position)
            .and_then(|objects| objects.iter_mut().find(|obj| matches!(obj, MapObject::Exit(_))))
        {
            *map_object = MapObject::Exit(level_name.to_string());
        } else {
            self.objects.entry(position).or_default().push(MapObject::Exit(level_name.to_string()));
        }
    }

    pub fn remove_exit(&mut self, position: (i32, i32)) {
        if let Some(objects) = self.objects.get_mut(&position) {
            objects.retain(|obj| !matches!(obj, MapObject::Exit(_)));
            if objects.is_empty() {
                self.objects.remove(&position);
            }
        }
    }

    pub fn add_enemy(&mut self, position: (i32, i32), enemy_id: impl ToString) {
        if let Some(map_object) = self
            .objects
            .get_mut(&position)
            .and_then(|objects| objects.iter_mut().find(|obj| matches!(obj, MapObject::Enemy(_))))
        {
            *map_object = MapObject::Enemy(enemy_id.to_string());
        } else {
            self.objects.entry(position).or_default().push(MapObject::Enemy(enemy_id.to_string()));
        }
    }

    pub fn remove_enemy(&mut self, position: (i32, i32)) {
        if let Some(objects) = self.objects.get_mut(&position) {
            objects.retain(|obj| !matches!(obj, MapObject::Enemy(_)));
            if objects.is_empty() {
                self.objects.remove(&position);
            }
        }
    }

    pub fn add_item(&mut self, position: (i32, i32), item_id: impl ToString) {
        if let Some(map_object) = self
            .objects
            .get_mut(&position)
            .and_then(|objects| objects.iter_mut().find(|obj| matches!(obj, MapObject::Item(_))))
        {
            *map_object = MapObject::Item(item_id.to_string());
        } else {
            self.objects.entry(position).or_default().push(MapObject::Item(item_id.to_string()));
        }
    }

    pub fn remove_item(&mut self, position: (i32, i32)) {
        if let Some(objects) = self.objects.get_mut(&position) {
            objects.retain(|obj| !matches!(obj, MapObject::Item(_)));
            if objects.is_empty() {
                self.objects.remove(&position);
            }
        }
    }
}
