#![allow(dead_code)]

use bevy::prelude::*;

use crate::scenes::battlefield::damage::Damage;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, death);
    }
}

#[derive(Component, Clone)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn take_damage(&mut self, amount: Damage) {
        self.current -= amount.amount;
    }

    pub fn heal(&mut self, amount: f32) {
        self.current += amount;
        if self.current > self.max {
            self.current = self.max;
        }
    }
}

#[derive(EntityEvent)]
pub struct KilledEvent {
    pub entity: Entity,
}

fn death(mut commands: Commands, mut query: Query<(Entity, &Health)>) {
    for (entity, health) in query.iter_mut() {
        if health.current <= 0.0 {
            commands.trigger(KilledEvent { entity });
            commands.entity(entity).despawn();
        }
    }
}
