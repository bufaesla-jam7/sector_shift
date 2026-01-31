use bevy::prelude::*;

use crate::actors::enemy_controller::EnemyControllerPlugin;

pub struct ActorsPlugin;
impl Plugin for ActorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnemyControllerPlugin);
    }
}
