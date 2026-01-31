use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
/// Abstracts enemy movement, acting, and animation, also controlling the rotation
pub struct EnemyActionController {
    /// When an enemy attacks, it will not move during that period but continue after it finished
    /// attacking unless it is reset to [`MovementState::Idle`].
    /// Movement is relative to the rotation of the enemy, which can be controlled through
    /// [`Self::look_at`].
    pub movement_state: MovementState,
    rotation: Quat,
    /// Used to prevent the AI from having to care about attributes like the time an action takes
    action_request: Option<EnemyAction>,
    /// The action that the enemy is currently performing, if any
    action_state: Option<(Timer, EnemyAction)>,
}

#[derive(Reflect)]
/// A movement relative to the enemies rotation as set in the controller, not the actual
/// [`Transform`] rotation that can lag behind to prevent snapping
pub enum MovementState {
    Idle,
    /// Running forward
    Forward,
    /// Running backward
    Backward,
    /// Running to sidewards, to the left
    Left,
    /// Running to sidewards, to the right
    Right,
}

#[derive(Reflect)]
pub enum EnemyAction {
    PrimaryAttack,
    SecondaryAttack,
}

impl EnemyActionController {
    /// Rotate the enemy towards `position`.
    /// `position` does not have to be normalized and can be zero
    pub fn look_at(&mut self, position: Vec3) {
        self.rotation = Quat::from_rotation_arc(Vec3::Z, position.with_y(0.).normalize_or(Vec3::Z));
    }

    /// Perform an action
    /// This will only have an effect if the enemy is not already performing an action
    pub fn act(&mut self, action: EnemyAction) {
        if self.action_state.is_none() {
            self.action_request = Some(action);
        }
    }

    /// Returns true if the enemy is currently performing an action
    pub fn is_acting(&self) -> bool {
        self.action_state.is_none() && self.action_request.is_none()
    }
}
