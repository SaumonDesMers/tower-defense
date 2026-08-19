use avian2d::prelude::*;
use bevy::prelude::*;

use crate::scenes::AppState;
use crate::scenes::battlefield::health::{Damage, Health};

use super::base::Base;
use super::pathfinding::PathfindingMap;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enemy_ai_system.run_if(in_state(AppState::InGame)))
            .add_systems(
                Update,
                enemy_reached_target_system.run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Component, Clone)]
pub struct Enemy;

#[derive(Component, Clone)]
pub struct EnemyAI {
    pub speed: f32,
}

fn enemy_ai_system(
    mut enemy_ai_query: Query<(&EnemyAI, &Transform, &mut LinearVelocity)>,
    pathfinding_map: Res<PathfindingMap>,
) {
    for (enemy_ai, transform, mut velocity) in enemy_ai_query.iter_mut() {
        velocity.0 = pathfinding_map.get_flow(
            Vec2::new(transform.translation.x, transform.translation.y),
            10.0,
        ) * enemy_ai.speed;
    }
}

fn enemy_reached_target_system(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
    mut target_query: Query<(Entity, &mut Health), With<Base>>,
    collisions: Collisions,
) {
    for enemy_entity in enemy_query.iter() {
        for (target_entity, mut health) in target_query.iter_mut() {
            if collisions.contains(enemy_entity, target_entity) {
                health.take_damage(Damage::new(1));
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}
