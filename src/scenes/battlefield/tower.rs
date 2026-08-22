use std::sync::Arc;
use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;

use super::buildings::Building;
use super::enemy::Enemy;
use super::health::Health;
use super::physic::GameLayer;
use super::projectile::Projectile;
use super::selection::Selectable;
use crate::RessourcesHandler;
use crate::scenes::AppState;
use crate::scenes::battlefield::BattleFieldSet;
use crate::scenes::battlefield::attack_range::{AttackRange, AttackRangeType};
use crate::scenes::battlefield::attack_speed::AttackSpeed;
use crate::scenes::battlefield::currency::Coins;
use crate::scenes::battlefield::damage::Damage;
use crate::scenes::battlefield::projectile::ProjectileFiredEvent;
use crate::scenes::battlefield::projectile::ricochet::{Ricochet, SendProjectileWithRicochet};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(buy_tower.run_if(in_state(AppState::InGame)))
            .add_systems(Update, tower_system.in_set(BattleFieldSet));
    }
}

#[derive(Component)]
pub struct Tower;

#[derive(Resource)]
pub struct TowerGlobalData {
    pub build_price: f32,
    pub upgrade_price: f32,
}

#[derive(Event)]
pub struct BuyTowerEvent;

pub fn tower(ressources_handler: &RessourcesHandler) -> impl Bundle {
    (
        DespawnOnExit(AppState::InGame),
        Transform::from_xyz(0.0, 0.0, 0.1),
        Mesh2d(ressources_handler.tower_body_mesh.clone()),
        MeshMaterial2d(ressources_handler.tower_body_material.clone()),
        Collider::circle(20.0),
        CollisionLayers::new(
            [GameLayer::Default, GameLayer::Building],
            GameLayer::Default,
        ),
        RigidBody::Kinematic,
        Building,
        Selectable,
        Name("Tower".into()),
        Tower,
        Damage::new(5.0),
        AttackSpeed::new(1.0),
        AttackRange::new(AttackRangeType::Circle(200.0)),
    )
}

fn tower_system(
    mut commands: Commands,
    mut tower_query: Query<
        (
            Entity,
            &Damage,
            &mut AttackSpeed,
            &AttackRange,
            &GlobalTransform,
        ),
        With<Tower>,
    >,
    target_query: Query<(Entity, &GlobalTransform), (With<Collider>, With<Enemy>)>,
    collisions: Collisions,
    time: Res<Time>,
    ressources_handler: Res<RessourcesHandler>,
) {
    for (tower_entity, damage, mut attack_speed, attack_range, tower_transform) in
        tower_query.iter_mut()
    {
        attack_speed.timer.tick(time.delta());
        if !attack_speed.timer.is_finished() {
            continue;
        }
        attack_speed.timer.reset();

        if let Some((target_entity, _target_transform)) = target_query
            .iter()
            .filter(|(entity, _target_transform)| {
                collisions.contains(attack_range.collider_entity, *entity)
            })
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
                    Projectile::new(target_entity, 500.0, *damage),
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

fn buy_tower(
    _: On<BuyTowerEvent>,
    mut commands: Commands,
    ressources_handler: Res<RessourcesHandler>,
    mut coins: ResMut<Coins>,
    mut tower_data: ResMut<TowerGlobalData>,
) {
    if coins.0 >= tower_data.build_price {
        coins.0 -= tower_data.build_price;
        tower_data.build_price *= 1.5;
        commands.spawn(tower(&ressources_handler));
    }
}
