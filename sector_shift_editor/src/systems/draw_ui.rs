use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use sector_shift_core::prelude::*;

use crate::{
    data::BrushType,
    resources::{BrushData, MapData, UiState},
};

pub fn draw_ui(
    mut contexts: EguiContexts,
    mut map_data: ResMut<MapData>,
    mut brush_data: ResMut<BrushData>,
    mut ui_state: ResMut<UiState>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };
    egui::Window::new("SectorShift Editor").default_width(200.0).show(ctx, |ui| {
        ui.heading("Map Name:");
        ui.text_edit_singleline(&mut ui_state.level_name);
        ui.separator();

        ui.heading("Brush Settings");

        ui.label("Brush Select:");
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut brush_data.brush,
                BrushType::Tile(TileType::Floor),
                "Floor",
            );

            ui.selectable_value(
                &mut brush_data.brush,
                BrushType::Tile(TileType::Wall),
                "Wall",
            );

            let is_door = matches!(brush_data.brush, BrushType::Tile(TileType::Door(_)));
            if ui.selectable_label(is_door, "Door").clicked() {
                brush_data.brush = BrushType::Tile(TileType::Door(ui_state.selected_door_axis));
            }

            let is_player_start = matches!(brush_data.brush, BrushType::PlayerStart(_));
            if ui.selectable_label(is_player_start, "Player Start").clicked() {
                brush_data.brush = BrushType::PlayerStart(ui_state.player_start_direction);
            }

            let is_item = matches!(brush_data.brush, BrushType::Item(_));
            if ui.selectable_label(is_item, "Item").clicked() {
                brush_data.brush = BrushType::Item(ui_state.item_name.clone());
            }

            let is_enemy = matches!(brush_data.brush, BrushType::Enemy(_));
            if ui.selectable_label(is_enemy, "Enemy").clicked() {
                brush_data.brush = BrushType::Enemy(ui_state.enemy_name.clone());
            }

            let is_exit = matches!(brush_data.brush, BrushType::Exit(_));
            if ui.selectable_label(is_exit, "Exit").clicked() {
                brush_data.brush = BrushType::Exit(ui_state.exit_name.clone());
            }

            ui.selectable_value(
                &mut brush_data.brush,
                BrushType::EraseObject,
                "Erase Object",
            );
        });

        if let BrushType::Tile(TileType::Door(_)) = brush_data.brush {
            ui.horizontal(|ui| {
                ui.label("Door Axis:");
                let mut changed = false;
                changed |= ui
                    .radio_value(
                        &mut ui_state.selected_door_axis,
                        DoorAxis::Vertical,
                        "Vertical",
                    )
                    .changed();
                changed |= ui
                    .radio_value(
                        &mut ui_state.selected_door_axis,
                        DoorAxis::Horizontal,
                        "Horizontal",
                    )
                    .changed();
                if changed {
                    brush_data.brush = BrushType::Tile(TileType::Door(ui_state.selected_door_axis));
                }
            });
        } else if let BrushType::PlayerStart(_direction) = brush_data.brush {
            ui.horizontal(|ui| {
                ui.label("Player Start Direction:");
                let mut changed = false;
                changed |= ui
                    .radio_value(
                        &mut ui_state.player_start_direction,
                        Direction::NORTH,
                        "North",
                    )
                    .changed();
                changed |= ui
                    .radio_value(
                        &mut ui_state.player_start_direction,
                        Direction::SOUTH,
                        "South",
                    )
                    .changed();
                changed |= ui
                    .radio_value(
                        &mut ui_state.player_start_direction,
                        Direction::EAST,
                        "East",
                    )
                    .changed();
                changed |= ui
                    .radio_value(
                        &mut ui_state.player_start_direction,
                        Direction::WEST,
                        "West",
                    )
                    .changed();
                if changed {
                    brush_data.brush = BrushType::PlayerStart(ui_state.player_start_direction);
                }
            });
        } else if let BrushType::Item(_) = brush_data.brush {
            ui.horizontal(|ui| {
                ui.label("Item Name:");
                if ui.text_edit_singleline(&mut ui_state.item_name).changed() {
                    brush_data.brush = BrushType::Item(ui_state.item_name.clone());
                }
            });
        } else if let BrushType::Enemy(_) = brush_data.brush {
            ui.horizontal(|ui| {
                ui.label("Enemy Name:");
                if ui.text_edit_singleline(&mut ui_state.enemy_name).changed() {
                    brush_data.brush = BrushType::Enemy(ui_state.enemy_name.clone());
                }
            });
        } else if let BrushType::Exit(_) = brush_data.brush {
            ui.horizontal(|ui| {
                ui.label("Exit Name:");
                if ui.text_edit_singleline(&mut ui_state.exit_name).changed() {
                    brush_data.brush = BrushType::Exit(ui_state.exit_name.clone());
                }
            });
        }

        ui.separator();

        ui.horizontal(|ui| {
            // SAVE BUTTON
            if ui.button("Save Map").clicked() {
                map_data.level.id = ui_state.level_name.clone();
                if let Err(e) = map_data.level.save() {
                    ui_state.status_message = format!("Error Saving Map: {}", e);
                } else {
                    ui_state.status_message = "Map Saved Successfully".to_string();
                }
            }

            // LOAD BUTTON
            if ui.button("Load Map").clicked() {
                if let Ok(level) = Level::try_load(ui_state.level_name.clone()) {
                    map_data.level = level;
                    ui_state.status_message = "Map Loaded Successfully".to_string();
                } else {
                    ui_state.status_message = "Error Loading Map".to_string();
                }
            }
        });
    });
}
