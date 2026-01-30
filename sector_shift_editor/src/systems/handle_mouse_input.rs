use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContexts;

use crate::{
    CELL_SIZE,
    data::BrushType,
    resources::{BrushData, MapData},
};

pub fn handle_mouse_input(
    mut contexts: EguiContexts,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut map_data: ResMut<MapData>,
    brush_data: Res<BrushData>,
    q_window: Single<&Window, With<PrimaryWindow>>,
    q_camera: Single<(&Camera, &GlobalTransform)>,
) {
    // Don't click through
    if let Ok(ctx) = contexts.ctx_mut()
        && (ctx.is_pointer_over_area() || ctx.wants_pointer_input())
    {
        return;
    }

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
                BrushType::Tile(tile_type) => {
                    map_data.level.set_tile(position, *tile_type);
                },
                BrushType::Enemy(id) => map_data.level.add_enemy(position, id),
                BrushType::Exit(id) => map_data.level.add_exit(position, id),
                BrushType::Item(id) => map_data.level.add_item(position, id),
                BrushType::EraseObject => map_data.level.remove_object(position),
            }
        }
    }
}
