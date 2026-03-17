use bevy::prelude::*;
use avian2d::prelude::*;

use crate::RessourcesHandler;
use super::selection::Selectable;
use super::physic::GameLayer;
use super::buildings::Building;
use super::BattleField;

pub fn obstacle(
	ressources_handler: &RessourcesHandler,
) -> impl Bundle {
	(
        BattleField,
        Transform::from_xyz(-400.0, 100.0, 0.0)
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2 / 2.0)),
        Mesh2d(ressources_handler.obstacle_mesh.clone()),
        MeshMaterial2d(ressources_handler.obstacle_material.clone()),
        Collider::rectangle(50.0, 200.0),
        RigidBody::Static,
        CollisionLayers::new([GameLayer::Default, GameLayer::Building], GameLayer::Default),
        Building,
        Selectable,
    )
}

