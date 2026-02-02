use std::{ops::Range, time::Duration};

pub struct EnemyAttack {
    pub range: Range<f32>,
    pub duration: Duration,
}
