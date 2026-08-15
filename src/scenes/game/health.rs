#![allow(dead_code)]

use bevy::prelude::*;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, death);
    }
}

#[derive(Component, Clone)]
pub struct Health {
    current: i32,
    max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }

    pub fn take_damage(&mut self, amount: Damage) {
        self.current -= amount.amount;
    }

    pub fn heal(&mut self, amount: i32) {
        self.current += amount;
        if self.current > self.max {
            self.current = self.max;
        }
    }
}

fn death(mut commands: Commands, mut query: Query<(Entity, &Health)>) {
    for (entity, health) in query.iter_mut() {
        if health.current <= 0 {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Damage {
    pub amount: i32,
}

impl Damage {
    pub fn new(amount: i32) -> Self {
        Self { amount }
    }
}
