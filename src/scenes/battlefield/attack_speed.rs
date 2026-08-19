use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct AttackSpeed {
    pub timer: Timer,
}

impl AttackSpeed {
    pub fn new(per_second: f32) -> Self {
        Self {
            timer: Timer::new(Duration::from_secs_f32(1.0 / per_second), TimerMode::Once),
        }
    }

    pub fn per_second(&self) -> f32 {
        1.0 / self.timer.duration().as_secs_f32()
    }
}
