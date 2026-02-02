use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
/// see [`EnemyMovementAnimationInfo`]
pub(crate) struct EnemyMovementAnimationNames {
    pub idle: String,

    #[serde(default = "one")]
    pub idle_playback_speed: f32,

    pub walk_forwards: String,

    #[serde(default = "one")]
    pub forward_playback_speed: f32,

    #[serde(default)]
    pub walk_backwards: Option<String>,

    #[serde(default = "one")]
    pub backward_playback_speed: f32,

    #[serde(default)]
    pub walk_left: Option<String>,

    #[serde(default = "one")]
    pub left_playback_speed: f32,

    #[serde(default)]
    pub walk_right: Option<String>,

    #[serde(default = "one")]
    pub right_playback_speed: f32,
}

/// The serde derive does not allow providing default values inline, they have to be set via
/// [`Default::default()`] or a function.
fn one() -> f32 {
    1.
}
