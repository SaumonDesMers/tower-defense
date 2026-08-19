use std::time::Duration;

use avian2d::collision::collider::{Collider, Sensor};
use bevy::prelude::*;

use crate::RessourcesHandler;

pub struct AttackRangePlugin;

impl Plugin for AttackRangePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_range);
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

fn on_add_range(
    event: On<Add, AttackRange>,
    mut commands: Commands,
    mut query: Query<&mut AttackRange>,
    ressources_handler: Res<RessourcesHandler>,
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
            Mesh2d(ressources_handler.tower_range_mesh.clone()),
            MeshMaterial2d(ressources_handler.tower_range_material.clone()),
            Pickable::IGNORE,
        ))
        .id();
    cmd.add_child(id);
    range.collider_entity = id;
}
