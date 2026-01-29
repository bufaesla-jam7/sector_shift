# 3d FPS based on classics like Wolfenstein3d

# SECTOR_SHIFT_CORE
Data and utilities only. Meant to hold shared information between the editor and the game.

## TODO:
- Need Weapon Asset System (like items and enemies)
- Need Health components
- Need Enemy AI

# SECTOR_SHIFT_EDITOR
A basic map editor

`cargo run -p sector_shift_editor --features dev`

## TODO:
- Want build random map

# SECTOR_SHIFT_GAME
The main game

Build a "level_1" first using the editor making sure to place the player start with other floor tiles to make room to move.

`cargo run -p sector_shift_game --features dev`

## TODO:
- Build out items plugin including components / functions (spawn_item) / Weapons
- Load MapObjects from `level.objects` in  `crate::maps::functions::spawn_level`
- Change mesh/standardmaterial to sprites?
- Ceiling + lighting could be thought about
- Map transition on enter `TileType::Exit(id)` Collider + Sensor components
- Door functionality
- Player UI