use sector_shift_core::prelude::*;

#[derive(PartialEq, Eq)]
pub enum BrushType {
    PlayerStart(Direction),

    Enemy(String),
    EraseEnemy,

    Exit(String),
    EraseExit,

    Item(String),
    EraseItem,

    Tile(TileType),
}
