use bevy::prelude::*;
use sector_shift_core::prelude::TileType;

use crate::{CELL_SIZE, HALF_CELL_SIZE, resources::MapData};

pub fn draw_grid(mut gizmos: Gizmos, editor: Res<MapData>) {
    // Draw vertical lines
    let height = editor.level.height() as f32 * CELL_SIZE;
    for x in 0..=editor.level.width() {
        let x_position = x as f32 * CELL_SIZE;

        gizmos.line(
            Vec3::new(x_position, 0.0, 0.0),
            Vec3::new(x_position, height, 0.0),
            Color::srgb(0.2, 0.2, 0.2),
        );
    }

    // Draw horizontal lines
    let width = editor.level.width() as f32 * CELL_SIZE;
    for y in 0..=editor.level.height() {
        let y_position = y as f32 * CELL_SIZE;

        gizmos.line(
            Vec3::new(0.0, y_position, 0.0),
            Vec3::new(width, y_position, 0.0),
            Color::srgb(0.2, 0.2, 0.2),
        );
    }

    // Draw filled boxes
    for (position, tile) in editor.level.tiles.enumerate() {
        if *tile == TileType::Floor {
            continue;
        }

        let center = Vec2::new(
            position.0 as f32 * CELL_SIZE + HALF_CELL_SIZE,
            position.1 as f32 * CELL_SIZE + HALF_CELL_SIZE,
        );

        gizmos.rect_2d(center, Vec2::splat(CELL_SIZE * 0.9), tile.color());
    }
}
