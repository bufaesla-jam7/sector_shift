# 3d FPS based on classics like Wolfenstein3d

## Run the editor:
`cargo run -p sector_shift_editor --features dev`

# SECTOR_SHIFT_CORE
Data and utilities only. Meant to hold shared information between the editor and the game.

## TODO:
- Need Weapon Asset System (like items and enemies)
- Need Health components
- Need Enemy AI

# SECTOR_SHIFT_EDITOR
A basic map editor

## TODO:
- Need visual representation of `MapObjects` on the map.
- Want build random map

# SECTOR_SHIFT_GAME
The main game

## TODO:
- Build maps with graphics / Colliders
- Map transition on enter `TileType::Exit(id)`
- Door functionality
- Player FPS controller
- Player UI