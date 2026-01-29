use bevy::prelude::*;

use crate::items::data::ItemEffect;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Item {
    pub effect: ItemEffect,
}
