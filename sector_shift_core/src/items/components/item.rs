use bevy::prelude::*;

use crate::items::data::ItemEffect;

/// Item component
/// This holds the item's pickup effect
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Item {
    /// The effect this item has when picked up
    pub effect: ItemEffect,
}
