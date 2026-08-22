use avian2d::prelude::*;
use bevy::prelude::*;

use super::buildings::Building;
use super::physic::GameLayer;
use super::selection::Selectable;
use crate::RessourcesHandler;
use crate::scenes::AppState;
use crate::scenes::battlefield::currency::Coins;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(buy_obstacle.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Resource)]
pub struct ObstacleGlobalData {
    pub price: f32,
}

#[derive(Event)]
pub struct BuyObstacleEvent;

fn buy_obstacle(
    _: On<BuyObstacleEvent>,
    mut commands: Commands,
    ressources_handler: Res<RessourcesHandler>,
    mut coins: ResMut<Coins>,
    mut obstacle_data: ResMut<ObstacleGlobalData>,
) {
    if coins.0 >= obstacle_data.price {
        coins.0 -= obstacle_data.price;
        obstacle_data.price *= 1.5;
        commands.spawn(obstacle(&ressources_handler));
    }
}

pub fn obstacle(ressources_handler: &RessourcesHandler) -> impl Bundle {
    (
        DespawnOnExit(AppState::InGame),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Mesh2d(ressources_handler.obstacle_mesh.clone()),
        MeshMaterial2d(ressources_handler.obstacle_material.clone()),
        Collider::rectangle(50.0, 200.0),
        CollisionLayers::new(
            [GameLayer::Default, GameLayer::Building],
            GameLayer::Default,
        ),
        RigidBody::Kinematic,
        Building,
        Selectable,
        Name("Obstacle".into()),
    )
}
