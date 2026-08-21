use std::{marker::PhantomData, time::Duration};

use bevy::prelude::*;

use crate::scenes::battlefield::upgrade::Upgrade;

pub struct AttackSpeedPlugin;

impl Plugin for AttackSpeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_attack_speed_upgrade);
    }
}

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

#[derive(EntityEvent, Clone)]
pub struct AttackSpeedUpgrade {
    entity: Entity,
}

impl AttackSpeedUpgrade {
    pub fn new() -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
        }
    }
}

impl Upgrade for AttackSpeedUpgrade {
    fn trigger(&self, commands: &mut Commands, entity: Entity) {
        commands.trigger(Self { entity, ..*self });
    }

    fn text(&self) -> String {
        String::from("Attack speed: +10%")
    }
}

fn on_attack_speed_upgrade(
    event: On<AttackSpeedUpgrade>,
    mut attack_speed_q: Query<&mut AttackSpeed>,
) {
    info!("on_attack_speed_upgrade");
    if let Ok(mut attack_speed) = attack_speed_q.get_mut(event.entity) {
        *attack_speed = AttackSpeed::new(attack_speed.per_second() * 1.1);
    }
}
