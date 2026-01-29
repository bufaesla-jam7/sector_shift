use bevy::prelude::*;
use sector_shift_core::{maps::MapObject, prelude::TileType};

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

    // Draw tiles
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

    // Draw objects
    for (position, object) in editor.level.objects.iter() {
        let center = Vec2::new(
            position.0 as f32 * CELL_SIZE + HALF_CELL_SIZE,
            position.1 as f32 * CELL_SIZE + HALF_CELL_SIZE,
        );

        match object {
            MapObject::Exit(_) => {
                gizmos.cross_2d(center, HALF_CELL_SIZE * 0.8, Color::srgb(0.1, 0.4, 0.8));
            },
            MapObject::Enemy(_) => {
                let coord = Vec2::new(HALF_CELL_SIZE * 0.8, HALF_CELL_SIZE * 0.8);
                let start = center - coord;
                let end = center + coord;
                gizmos.line_2d(start, end, Color::srgb(0.8, 0.1, 0.1));
            },
            MapObject::Item(_) => {
                gizmos.circle_2d(center, HALF_CELL_SIZE * 0.8, Color::srgb(0.1, 0.8, 0.1));
            },
        }
    }

    // Draw player start
    let (position, direction) = editor.level.player_start;
    let center = Vec2::new(
        position.0 as f32 * CELL_SIZE + HALF_CELL_SIZE,
        position.1 as f32 * CELL_SIZE + HALF_CELL_SIZE,
    );
    let coord = Vec2::new(
        direction.coord().0 as f32 * HALF_CELL_SIZE * 0.8,
        direction.coord().1 as f32 * HALF_CELL_SIZE * 0.8,
    );
    let start = center - coord;
    let end = center + coord;
    gizmos.arrow_2d(start, end, Color::srgb(0.1, 0.8, 0.1));
}
