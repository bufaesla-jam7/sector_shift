use bevy::transform::components::Transform;

use crate::{MAP_CELL_CEILING, MAP_CELL_HEIGHT, MAP_CELL_WIDTH};

pub fn position_to_transform(position: (i32, i32)) -> Transform {
    Transform::from_xyz(
        MAP_CELL_WIDTH * (position.0 as f32 + 0.5),
        MAP_CELL_CEILING / 2.0,
        MAP_CELL_HEIGHT * (position.1 as f32 + 0.5),
    )
}
