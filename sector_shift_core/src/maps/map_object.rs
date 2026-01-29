use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Reflect, Debug, Clone)]
pub enum MapObject {
    Exit(String),
    Enemy(String),
    Item(String),
}
