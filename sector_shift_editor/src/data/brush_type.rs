use sector_shift_core::prelude::*;

#[derive(PartialEq, Eq)]
pub enum BrushType {
    PlayerStart(Direction),

    Tile(TileType),

    Enemy(String),
    Exit(String),
    Item(String),

    EraseObject,
}
