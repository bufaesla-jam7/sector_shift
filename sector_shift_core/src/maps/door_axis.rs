use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Doors take up a cell and either connect North to South or East to West.
#[derive(Serialize, Deserialize, Reflect, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DoorAxis {
    /// Doors that connect East to West.
    #[default]
    Horizontal,
    /// Doors that connect North to South.
    Vertical,
}
