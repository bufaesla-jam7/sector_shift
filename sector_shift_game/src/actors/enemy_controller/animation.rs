use bevy::prelude::*;
use sector_shift_core::{
    enemies::components::{EnemyAnimationGraphTempStorage, EnemyAnimationTarget, EnemyArmature},
    prelude::{Enemy, EnemyLibrary},
};

use crate::actors::enemy_controller::{EnemyAction, EnemyController, MovementState};

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
    library: Res<EnemyLibrary>,
    enemies: Query<(&Name, &EnemyAnimationTarget, &mut EnemyController)>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions), With<EnemyArmature>>,
) {
    for (name, animation_target, mut controller) in enemies {
        let movement_animations = &library.map[name.as_str()].movement_animation_info;
        let action_info = &library.map[name.as_str()].action_info;

        let Ok((mut animation_player, mut animation_transitions)) =
            animation_players.get_mut(animation_target.0)
        else {
            warn!("EnemyController has an invalid EnemyAnimationTarget");
            continue;
        };
        if controller.movement_changed || animation_player.all_finished() {
            controller.movement_changed = false;

            let ((animation_index, speed), is_repeating);

            // The enemy is attacking
            if let Some((_, action)) = &controller.action_state {
                (animation_index, speed) = match action {
                    EnemyAction::PrimaryAttack => (
                        action_info.primary_attack.animation_index,
                        action_info.primary_attack.animation_playback_speed,
                    ),
                    EnemyAction::SecondaryAttack => {
                        let attack = action_info
                            .secondary_attack
                            .as_ref()
                            .expect("Guaranteed to exist by the EnemyController API");
                        (attack.animation_index, attack.animation_playback_speed)
                    },
                };
                // Do not repeat the attacking animation
                is_repeating = false;
            // The enemy is moving
            } else {
                (animation_index, speed) = match controller.movement_state {
                    MovementState::Idle => (
                        movement_animations.idle,
                        movement_animations.idle_playback_speed,
                    ),
                    MovementState::Forward => (
                        movement_animations.walk_forwards,
                        movement_animations.forward_playback_speed,
                    ),
                    MovementState::Backward => match movement_animations.walk_backwards {
                        // Use backward animation if available
                        Some(index) => (index, movement_animations.backward_playback_speed),
                        // Fallback to inverse forward animation
                        None => (
                            movement_animations.walk_forwards,
                            -movement_animations.backward_playback_speed,
                        ),
                    },
                    MovementState::Left => match movement_animations.walk_left {
                        // Use left animation if available
                        Some(index) => (index, movement_animations.left_playback_speed),
                        None => match movement_animations.walk_right {
                            // Fallback to inverse right animation if available
                            Some(index) => (index, -movement_animations.left_playback_speed),
                            // Fallback to just playing idle
                            None => (
                                movement_animations.idle,
                                movement_animations.left_playback_speed,
                            ),
                        },
                    },
                    MovementState::Right => match movement_animations.walk_right {
                        // Use right animation if available
                        Some(index) => (index, movement_animations.right_playback_speed),
                        None => match movement_animations.walk_left {
                            // Fallback to inverse left animation if available
                            Some(index) => (index, -movement_animations.right_playback_speed),
                            // Fallback to just playing idle
                            None => (
                                movement_animations.idle,
                                movement_animations.right_playback_speed,
                            ),
                        },
                    },
                };
                // Repeat the movement animation
                is_repeating = true;
            };
            let active_animation = animation_transitions
                .play(
                    &mut animation_player,
                    animation_index,
                    controller.animation_transition_duration,
                )
                .set_speed(speed);
            if is_repeating {
                active_animation.repeat();
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
        KeyCode::KeyQ,
        KeyCode::KeyE,
    ]) {
        for mut controller in query {
            if input.just_pressed(KeyCode::KeyQ) {
                controller.act(EnemyAction::PrimaryAttack);
                continue;
            }
            if input.just_pressed(KeyCode::KeyE) {
                controller.act(EnemyAction::SecondaryAttack);
                continue;
            }

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
