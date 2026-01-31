use bevy::prelude::*;

use crate::{actors::enemy_controller::systems::*, states::system_sets::GameSet};

pub struct EnemyControllerPlugin;

impl Plugin for EnemyControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (apply_rotation, apply_velocity).in_set(GameSet::Running),
        );
    }
}
