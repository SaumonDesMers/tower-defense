use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;

use super::BattleField;
use super::buildings::Building;
use super::enemy::Enemy;
use super::health::{Damage, Health};
use super::physic::GameLayer;
use super::projectile::{Projectile, ProjectileHitEvent};
use super::selection::Selectable;
use crate::RessourcesHandler;
use crate::scenes::game::projectile::ProjectileFiredEvent;
use crate::scenes::game::projectile::ricochet::{Ricochet, SendProjectileWithRicochet};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tower_system);
    }
}

#[derive(Component)]
pub struct Tower {
    pub attack_timer: Timer,
    pub damage: Damage,
}

impl Tower {
    pub fn new(attack_duration: Duration, damage: Damage) -> Self {
        Self {
            attack_timer: Timer::from_seconds(attack_duration.as_secs_f32(), TimerMode::Once),
            damage,
        }
    }
}

pub fn tower(ressources_handler: &RessourcesHandler) -> impl Bundle {
    (
        BattleField,
        Transform::from_xyz(0.0, 0.0, 0.1),
        Mesh2d(ressources_handler.tower_body_mesh.clone()),
        MeshMaterial2d(ressources_handler.tower_body_material.clone()),
        Collider::circle(20.0),
        CollisionLayers::new(
            [GameLayer::Default, GameLayer::Building],
            GameLayer::Default,
        ),
        Health::new(100),
        Building,
        Selectable,
        children![(
            Transform::default(),
            Tower::new(Duration::from_secs_f32(0.8), Damage::new(5)),
            Collider::circle(200.0),
            Mesh2d(ressources_handler.tower_range_mesh.clone()),
            MeshMaterial2d(ressources_handler.tower_range_material.clone()),
            SendProjectileWithRicochet,
        )],
    )
}

fn tower_system(
    mut commands: Commands,
    mut tower_query: Query<(Entity, &mut Tower, &GlobalTransform), With<Collider>>,
    target_query: Query<(Entity, &GlobalTransform), (With<Collider>, With<Enemy>)>,
    collisions: Collisions,
    time: Res<Time>,
    ressources_handler: Res<RessourcesHandler>,
) {
    for (tower_entity, mut tower, tower_transform) in tower_query.iter_mut() {
        tower.attack_timer.tick(time.delta());
        if !tower.attack_timer.is_finished() {
            continue;
        }
        tower.attack_timer.reset();

        if let Some((target_entity, _target_transform)) = target_query
            .iter()
            .filter(|(entity, _target_transform)| collisions.contains(tower_entity, *entity))
            .min_by_key(|(_entity, target_transform)| {
                tower_transform
                    .translation()
                    .distance(target_transform.translation()) as u64
            })
        {
            let projectile_entity = commands
                .spawn((
                    Transform::from_translation(tower_transform.translation()),
                    Mesh2d(ressources_handler.projectile_mesh.clone()),
                    MeshMaterial2d(ressources_handler.projectile_material.clone()),
                    Collider::circle(2.0),
                    Sensor,
                    RigidBody::Kinematic,
                    Projectile::new(target_entity, 500.0, tower.damage),
                ))
                .id();

            commands.trigger(ProjectileFiredEvent {
                entity: projectile_entity,
                source_entity: tower_entity,
                target_entity,
            });
        }
    }
}
