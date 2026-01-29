use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    CELL_SIZE,
    data::BrushType,
    resources::{BrushData, MapData},
};

pub fn handle_mouse_input(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut map_data: ResMut<MapData>,
    brush_data: Res<BrushData>,
    q_window: Single<&Window, With<PrimaryWindow>>,
    q_camera: Single<(&Camera, &GlobalTransform)>,
) {
    if mouse_input.pressed(MouseButton::Left) {
        let (camera, camera_transform) = *q_camera;
        let window = *q_window;

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor_pos| camera.viewport_to_world_2d(camera_transform, cursor_pos).ok())
        {
            let cell_x = (world_position.x / CELL_SIZE).floor() as i32;
            let cell_y = (world_position.y / CELL_SIZE).floor() as i32;

            let position = (cell_x, cell_y);

            match &brush_data.brush {
                BrushType::PlayerStart(direction) => map_data.level.set_player_start(position, *direction),
                BrushType::Enemy(id) => map_data.level.add_enemy(position, id),
                BrushType::EraseEnemy => map_data.level.remove_enemy(position),
                BrushType::Exit(id) => map_data.level.add_exit(position, id),
                BrushType::EraseExit => map_data.level.remove_exit(position),
                BrushType::Item(id) => map_data.level.add_item(position, id),
                BrushType::EraseItem => map_data.level.remove_item(position),
                BrushType::Tile(tile_type) => {
                    if let Some(tile) = map_data.level.tiles.get_mut(position) {
                        *tile = *tile_type;
                    }
                },
            }
        }
    }
}
