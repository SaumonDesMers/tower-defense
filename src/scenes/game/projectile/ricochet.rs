use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    RessourcesHandler,
    scenes::game::{
        enemy::Enemy,
        projectile::{Projectile, ProjectileFiredEvent, ProjectileHitEvent},
    },
};

/// Component for tower.
#[derive(Component, Debug, Clone, Copy)]
pub struct SendProjectileWithRicochet;

/// Component for projectile.
/// It store the child Entity which contains the collider needed to find a target.
#[derive(Component, Debug, Clone, Copy)]
pub struct Ricochet(Entity);

impl Ricochet {
    pub fn new() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}

/// When a entity with SendProjectileWithRicochet trigger a ProjectileFiredEvent, add:
/// - a child to the projectile entity with a Collider.
/// - a Ricochet component storing the id of the added child.
///
/// This Collider will be used by the ricochet systems to find a target.
pub fn on_send_projectile(
    event: On<ProjectileFiredEvent>,
    mut commands: Commands,
    query: Query<&SendProjectileWithRicochet>,
) {
    if let (Ok(_), Ok(mut projectile_cmd)) = (
        query.get(event.source_entity),
        commands.get_entity(event.entity),
    ) {
        let collider = projectile_cmd
            .commands_mut()
            .spawn((Collider::circle(100.0), Sensor))
            .id();
        projectile_cmd.add_child(collider);
        projectile_cmd.insert(Ricochet(collider));
    }
}

pub fn spawn(
    hit: On<ProjectileHitEvent>,
    mut commands: Commands,
    projectile_query: Query<(&Projectile, &Ricochet, &Children, &GlobalTransform)>,
    target_query: Query<(Entity, &GlobalTransform), (With<Collider>, With<Enemy>)>,
    collisions: Collisions,
    ressources_handler: Res<RessourcesHandler>,
) {
    let Ok((projectile, ricochet, children, transform)) = projectile_query.get(hit.entity) else {
        return;
    };

    let Some(collider_entity) = children.iter().find(|e| *e == ricochet.0) else {
        warn!("Entity with Collider for Ricochet doesn't exist.");
        return;
    };

    if let Some((target_entity, _)) = target_query
        .iter()
        .filter(|(entity, _target_transform)| *entity != hit.target_entity)
        .filter(|(entity, _target_transform)| collisions.contains(collider_entity, *entity))
        .min_by_key(|(_entity, target_transform)| {
            transform
                .translation()
                .distance(target_transform.translation()) as u64
        })
    {
        info!("Spawning ricochet");
        commands.spawn((
            Transform::from_translation(transform.translation()),
            Mesh2d(ressources_handler.projectile_mesh.clone()),
            MeshMaterial2d(ressources_handler.projectile_material.clone()),
            Collider::circle(2.0),
            Sensor,
            RigidBody::Kinematic,
            Projectile::new(target_entity, 500.0, projectile.damage),
        ));
    }
}
