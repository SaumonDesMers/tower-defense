#![allow(dead_code)]

use bevy::prelude::*;
use avian2d::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, projectile_system);
    }
}

#[derive(EntityEvent)]
pub struct ProjectileHitEvent {
    pub entity: Entity,
    pub target_entity: Entity,
}

#[derive(Component, Clone)]
pub struct Projectile {
    pub target: Entity,
    pub speed: f32,
    target_position: Vec2,
}

impl Projectile {
    pub fn new(target: Entity, speed: f32) -> Self {
        Self {
            target,
            speed,
            target_position: Vec2::ZERO,
        }
    }
}

fn projectile_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Projectile, &Transform, &mut LinearVelocity), With<Collider>>,
    target_query: Query<&Transform, With<Collider>>,
    collisions: Collisions,
) {
    for (entity, mut projectile, transform, mut velocity) in query.iter_mut() {
        projectile.target_position =
            if let Ok(target_transform) = target_query.get(projectile.target) {
                if collisions.contains(entity, projectile.target) {
                    commands.trigger(ProjectileHitEvent {
                        entity,
                        target_entity: projectile.target,
                    });
                    commands.entity(entity).despawn();
                }

                Vec2::new(
                    target_transform.translation.x,
                    target_transform.translation.y,
                )
            } else {
				if (projectile.target_position - Vec2::new(transform.translation.x, transform.translation.y)).length() < 10.0 {
                    commands.entity(entity).despawn();
                }
                projectile.target_position
            };

        let direction = (projectile.target_position - Vec2::new(transform.translation.x, transform.translation.y)).normalize_or_zero();
        velocity.0 = direction * projectile.speed;
    }
}
