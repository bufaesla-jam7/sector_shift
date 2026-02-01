use bevy::prelude::*;

use crate::{
    actors::enemy_controller::{animation, movement},
    states::system_sets::GameSet,
};

pub struct EnemyControllerPlugin;

impl Plugin for EnemyControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                movement::apply_rotation,
                movement::apply_velocity,
                animation::insert_targets,
                animation::animate_movement,
                animation::enemy_movement_test,
            )
                .in_set(GameSet::Running),
        );
    }
}
