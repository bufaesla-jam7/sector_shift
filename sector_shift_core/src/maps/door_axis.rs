use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Reflect, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DoorAxis {
    #[default]
    Horizontal,
    Vertical,
}
