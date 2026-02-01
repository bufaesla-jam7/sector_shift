use bevy::prelude::*;
use sector_shift_core::{
    enemies::components::{
        EnemyAnimationGraphTempStorage, EnemyAnimationTarget, EnemyArmature, EnemyMovementAnimationInfo,
    },
    prelude::Enemy,
};

use crate::actors::enemy_controller::{EnemyController, MovementState};

pub fn insert_targets(
    mut commands: Commands,
    parents: Query<
        (
            Has<Enemy>,
            Option<&EnemyAnimationGraphTempStorage>,
            Option<&ChildOf>,
        ),
        With<Children>,
    >,
    added: Query<Entity, Added<EnemyArmature>>,
) {
    for target_entity in added {
        let mut current_entity = target_entity;
        loop {
            if let Ok((is_enemy, maybe_graph_handle, maybe_parent)) = parents.get(current_entity) {
                if is_enemy {
                    commands.entity(current_entity).insert(EnemyAnimationTarget(target_entity));
                    if let Some(graph_handle) = maybe_graph_handle {
                        commands.entity(target_entity).insert((
                            AnimationGraphHandle(graph_handle.0.clone()),
                            AnimationTransitions::new(),
                        ));
                        commands.entity(current_entity).remove::<EnemyAnimationGraphTempStorage>();
                    } else {
                        warn!(
                            "Enemy entity {current_entity} does not have the \
                            EnemyAnimationGraphTempStorage component",
                        );
                    }
                    break;
                } else if let Some(parent) = maybe_parent {
                    current_entity = parent.0;
                    continue;
                }
            }
            warn!(
                "Entity {target_entity} with the EnemyArmature component does not have \
                a parent with the Enemy component",
            );
            break;
        }
    }
}

pub fn animate_movement(
    enemies: Query<(
        &EnemyMovementAnimationInfo,
        &EnemyAnimationTarget,
        &mut EnemyController,
    )>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions), With<EnemyArmature>>,
) {
    for (animations, animation_target, mut controller) in enemies {
        if controller.animation_changed {
            controller.animation_changed = false;
            if let Ok((mut animation_player, mut animation_transitions)) =
                animation_players.get_mut(animation_target.0)
            {
                if let Some((_, action)) = &controller.action_state {
                    todo!("animate enemy actions");
                } else {
                    let (animation_index, speed) = match controller.movement_state {
                        MovementState::Idle => (animations.idle, animations.idle_playback_speed),
                        MovementState::Forward => {
                            (animations.walk_forwards, animations.forward_playback_speed)
                        },
                        MovementState::Backward => match animations.walk_backwards {
                            // Use backward animation if available
                            Some(index) => (index, animations.backward_playback_speed),
                            // Fallback to inverse forward animation
                            None => (
                                animations.walk_forwards,
                                -animations.backward_playback_speed,
                            ),
                        },
                        MovementState::Left => match animations.walk_left {
                            // Use left animation if available
                            Some(index) => (index, animations.left_playback_speed),
                            None => match animations.walk_right {
                                // Fallback to inverse right animation if available
                                Some(index) => (index, -animations.left_playback_speed),
                                // Fallback to just playing idle
                                None => (animations.idle, animations.left_playback_speed),
                            },
                        },
                        MovementState::Right => match animations.walk_right {
                            // Use right animation if available
                            Some(index) => (index, animations.right_playback_speed),
                            None => match animations.walk_left {
                                // Fallback to inverse left animation if available
                                Some(index) => (index, -animations.right_playback_speed),
                                // Fallback to just playing idle
                                None => (animations.idle, animations.right_playback_speed),
                            },
                        },
                    };
                    animation_transitions
                        .play(
                            &mut animation_player,
                            animation_index,
                            controller.animation_transition_duration,
                        )
                        .repeat()
                        .set_speed(speed);
                }
            }
        }
    }
}

pub fn enemy_movement_test(input: Res<ButtonInput<KeyCode>>, query: Query<&mut EnemyController>) {
    if input.any_just_pressed([
        KeyCode::ArrowLeft,
        KeyCode::ArrowRight,
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
    ]) {
        for mut controller in query {
            let state = if input.just_pressed(KeyCode::ArrowUp) {
                MovementState::Forward
            } else if input.just_pressed(KeyCode::ArrowDown) {
                MovementState::Backward
            } else if input.just_pressed(KeyCode::ArrowLeft) {
                MovementState::Left
            } else {
                MovementState::Right
            };
            if *controller.movement() != state {
                controller.set_movement(state);
            } else {
                controller.set_movement(MovementState::Idle);
            }
        }
    }
}
