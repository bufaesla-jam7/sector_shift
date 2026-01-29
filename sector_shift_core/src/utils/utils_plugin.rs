use bevy::prelude::*;

use crate::utils::billboard::BillboardPlugin;

pub struct UtilsPlugin;
impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BillboardPlugin);
    }
}
