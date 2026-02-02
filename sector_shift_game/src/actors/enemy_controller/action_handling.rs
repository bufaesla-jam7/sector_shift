use bevy::prelude::*;
use sector_shift_core::prelude::EnemyLibrary;

use crate::actors::enemy_controller::{EnemyAction, EnemyController};

pub fn start_requested_actions(
    library: Res<EnemyLibrary>,
    controllers: Query<(&Name, &mut EnemyController)>,
) {
    for (name, mut controller) in controllers {
        let action_info = &library.map[name.as_str()].action_info;

        if let Some(action) = controller.action_request.take() {
            match action {
                EnemyAction::PrimaryAttack => {
                    controller.movement_changed = true;
                    controller.action_state = Some((
                        Timer::new(action_info.primary_attack.cast_duration, TimerMode::Once),
                        action,
                    ))
                },
                EnemyAction::SecondaryAttack => match &action_info.secondary_attack {
                    Some(attack) => {
                        controller.movement_changed = true;
                        controller.action_state =
                            Some((Timer::new(attack.cast_duration, TimerMode::Once), action))
                    },
                    None => warn!(
                        "enemy AI requested SecondaryAttack from an enemy that \
                        does not have a secondary attack"
                    ),
                },
            }
        }
    }
}

pub fn drive_action_timers(time: Res<Time>, controllers: Query<&mut EnemyController>) {
    for mut controller in controllers {
        if let Some((timer, _)) = &mut controller.action_state {
            timer.tick(time.delta());
            if timer.is_finished() {
                controller.action_state = None;
            }
        }
    }
}
