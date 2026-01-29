use bevy::prelude::*;

use crate::utils::billboard::systems::rotate_billboards;

pub struct BillboardPlugin;
impl Plugin for BillboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, rotate_billboards);
    }
}
