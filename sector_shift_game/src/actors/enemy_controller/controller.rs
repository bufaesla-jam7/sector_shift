use std::time::Duration;

use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
#[require(RigidBody::Dynamic, LockedAxes::ROTATION_LOCKED)]
/// Abstracts enemy movement, acting, and animation, also controlling the rotation, see:
/// - [`Self::set_movement`]
/// - [`Self::look_at`]
/// - [`Self::act`]
pub struct EnemyController {
    pub(super) movement_state: MovementState,
    pub(super) rotation: Quat,
    /// Used to signal the need for an animation change to the animation handling systems, which
    /// will reset this value to [`false`].
    pub(super) movement_changed: bool,
    /// Used to prevent the AI from having to care about attributes like the time an action takes
    pub(super) action_request: Option<EnemyAction>,
    /// The action that the enemy is currently performing, if any
    pub(super) action_state: Option<(Timer, EnemyAction)>,
    pub(super) animation_transition_duration: Duration,
}

#[derive(Reflect, PartialEq, Default, Debug)]
/// A movement relative to the enemies rotation as set in the controller, not the actual
/// [`Transform`] rotation that can lag behind to prevent snapping
pub enum MovementState {
    #[default]
    Idle,
    /// Running forward
    Forward,
    /// Running backward
    Backward,
    /// Running sideways, to the left
    Left,
    /// Running sideways, to the right
    Right,
}

#[derive(Reflect, Debug, Default)]
#[reflect(Default)]
pub enum EnemyAction {
    #[default]
    PrimaryAttack,
    SecondaryAttack,
}

impl EnemyController {
    /// Create a new [`EnemyController::default`] with a specific animation transition duration,
    /// specified in milliseconds
    pub fn new(transition_duration: u64) -> Self {
        Self {
            animation_transition_duration: Duration::from_millis(transition_duration),
            ..Default::default()
        }
    }

    /// Get the current movement state
    pub fn movement(&self) -> &MovementState {
        &self.movement_state
    }

    /// Rotate the enemy towards `position`.
    /// `position` does not have to be normalized and can be zero
    pub fn look_at(&mut self, position: Vec3) {
        self.rotation = Quat::from_rotation_arc(Vec3::Z, position.with_y(0.).normalize_or(Vec3::Z));
    }

    /// When an enemy attacks, it will not move during that period but continue after it finished
    /// attacking unless the movement state is set to [`MovementState::Idle`].
    /// Movement is relative to the rotation of the enemy, which can be controlled through
    /// [`Self::look_at`].
    pub fn set_movement(&mut self, movement: MovementState) {
        if self.movement_state != movement {
            self.movement_state = movement;
            self.movement_changed = true;
        }
    }

    /// Perform an action
    /// This will only have an effect if the enemy is not already performing an action
    pub fn act(&mut self, action: EnemyAction) {
        if self.action_state.is_none() {
            self.action_request = Some(action);
        }
    }

    /// Returns true if the enemy is currently performing an action (movement does not count as
    /// action)
    pub fn is_acting(&self) -> bool {
        self.action_state.is_some() || self.action_request.is_some()
    }
}
