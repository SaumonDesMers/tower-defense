use std::time::Duration;

use avian2d::collision::collider::{Collider, Sensor};
use bevy::{color::palettes::tailwind, prelude::*};

use crate::{RessourcesHandler, scenes::battlefield::upgrade::Upgrade};

pub struct AttackRangePlugin;

impl Plugin for AttackRangePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_insert_range)
            .add_observer(on_discard_range)
            .add_observer(on_attack_range_upgrade);
    }
}

#[derive(Component)]
pub struct AttackRange {
    pub collider_entity: Entity,
    pub range_type: AttackRangeType,
}

impl AttackRange {
    pub fn new(range_type: AttackRangeType) -> Self {
        Self {
            collider_entity: Entity::PLACEHOLDER,
            range_type,
        }
    }
}

pub enum AttackRangeType {
    Circle(f32),
}

#[derive(Component)]
struct AttackRangeCollider;

fn on_insert_range(
    event: On<Insert, AttackRange>,
    mut commands: Commands,
    mut query: Query<&mut AttackRange>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut cmd = commands.entity(event.entity);
    let mut range = query.get_mut(event.entity).unwrap();
    let id = cmd
        .commands()
        .spawn((
            Collider::from(match range.range_type {
                AttackRangeType::Circle(radius) => Circle::new(radius),
            }),
            Sensor,
            AttackRangeCollider,
            Mesh2d(meshes.add(match range.range_type {
                AttackRangeType::Circle(radius) => Circle::new(radius),
            })),
            MeshMaterial2d(materials.add(Color::from(tailwind::BLUE_800).with_alpha(0.2))),
            Pickable::IGNORE,
        ))
        .id();
    cmd.add_child(id);
    range.collider_entity = id;
}

fn on_discard_range(
    event: On<Discard, AttackRange>,
    mut commands: Commands,
    query: Query<&AttackRange>,
) {
    if let Ok(attack_range) = query.get(event.entity) {
        commands.entity(attack_range.collider_entity).despawn();
    }
}

#[derive(EntityEvent, Clone)]
pub struct AttackRangeUpgrade {
    entity: Entity,
}

impl AttackRangeUpgrade {
    pub fn new() -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
        }
    }
}

impl Upgrade for AttackRangeUpgrade {
    fn trigger(&self, commands: &mut Commands, entity: Entity) {
        commands.trigger(Self { entity, ..*self });
    }

    fn text(&self) -> String {
        String::from("attack_range: +10%")
    }
}

fn on_attack_range_upgrade(
    event: On<AttackRangeUpgrade>,
    mut commands: Commands,
    attack_range_q: Query<&AttackRange>,
) {
    if let Ok(attack_range) = attack_range_q.get(event.entity) {
        commands
            .entity(event.entity)
            .insert(match attack_range.range_type {
                AttackRangeType::Circle(radius) => {
                    AttackRange::new(AttackRangeType::Circle(radius * 1.1))
                }
            });
    }
}
