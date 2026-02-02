use bevy::prelude::*;

use crate::{
    actors::enemy_controller::{action_handling, animation, movement},
    states::system_sets::GameSet,
};

pub struct EnemyControllerPlugin;

impl Plugin for EnemyControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                // Movement
                movement::apply_rotation,
                movement::apply_velocity,
                // Animation
                animation::insert_targets,
                animation::animate_movement,
                animation::enemy_movement_test,
                // Action handling
                action_handling::start_requested_actions,
                action_handling::drive_action_timers,
            )
                .in_set(GameSet::Running),
        );
    }
}
