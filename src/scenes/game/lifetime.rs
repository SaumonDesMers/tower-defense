#![allow(dead_code)]

use std::time::Duration;

use bevy::prelude::*;

pub struct LifetimePlugin;

impl Plugin for LifetimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_lifetime);
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Lifetime {
    time_left: Duration,
}

impl Lifetime {
    pub fn new(duration: Duration) -> Self {
        Self {
            time_left: duration,
        }
    }
}

fn update_lifetime(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.time_left -= time.delta();
        if lifetime.time_left.is_zero() {
            commands.entity(entity).despawn();
        }
    }
}
