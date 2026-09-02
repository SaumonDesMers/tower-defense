#![allow(dead_code)]

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::scenes::battlefield::{damage::Damage, health::Health};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, projectile_move)
            .add_observer(projectile_apply_dammage);
    }
}

#[derive(EntityEvent)]
pub struct ProjectileFiredEvent {
    pub entity: Entity,
    pub source_entity: Entity,
    pub target_entity: Entity,
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
    pub damage: Damage,
}

impl Projectile {
    pub fn new(target: Entity, speed: f32, damage: Damage) -> Self {
        Self {
            target,
            speed,
            target_position: Vec2::ZERO,
            damage,
        }
    }
}

fn projectile_move(
    mut commands: Commands,
    mut projectile_query: Query<
        (Entity, &mut Projectile, &Transform, &mut LinearVelocity),
        With<Collider>,
    >,
    target_query: Query<&Transform, With<Collider>>,
    collisions: Collisions,
) {
    for (entity, mut projectile, transform, mut velocity) in projectile_query.iter_mut() {
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
                // If the target entity doesn't exist, travel to the last know position then despawn.
                if (projectile.target_position
                    - Vec2::new(transform.translation.x, transform.translation.y))
                .length()
                    < 10.0
                {
                    commands.entity(entity).despawn();
                }
                projectile.target_position
            };

        let direction = (projectile.target_position
            - Vec2::new(transform.translation.x, transform.translation.y))
        .normalize_or_zero();
        velocity.0 = direction * projectile.speed;
    }
}

fn projectile_apply_dammage(
    hit: On<ProjectileHitEvent>,
    projectile_query: Query<&Projectile>,
    mut target_query: Query<&mut Health>,
) {
    if let (Ok(projectile), Ok(mut health)) = (
        projectile_query.get(hit.entity),
        target_query.get_mut(hit.target_entity),
    ) {
        health.take_damage(projectile.damage);
    }
}
