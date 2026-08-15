use avian2d::prelude::*;
use bevy::prelude::*;

use super::BattleField;
use super::buildings::Building;
use super::physic::GameLayer;
use super::selection::Selectable;
use crate::RessourcesHandler;

pub fn obstacle(ressources_handler: &RessourcesHandler) -> impl Bundle {
    (
        BattleField,
        Transform::from_xyz(0.0, 0.0, 0.0),
        Mesh2d(ressources_handler.obstacle_mesh.clone()),
        MeshMaterial2d(ressources_handler.obstacle_material.clone()),
        Collider::rectangle(50.0, 200.0),
        RigidBody::Static,
        CollisionLayers::new(
            [GameLayer::Default, GameLayer::Building],
            GameLayer::Default,
        ),
        Building,
        Selectable,
    )
}
