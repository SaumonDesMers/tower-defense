use avian2d::prelude::*;
use bevy::prelude::*;

use super::BattleField;
use super::buildings::Building;
use super::physic::GameLayer;
use super::selection::Selectable;
use crate::RessourcesHandler;
use crate::scenes::SceneState;
use crate::scenes::battlefield::currency::Currency;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(buy_obstacle.run_if(in_state(SceneState::Battlefield)));
    }
}

#[derive(Resource)]
pub struct ObstacleGlobalData {
    pub price: u32,
}

#[derive(Event)]
pub struct BuyObstacleEvent;

fn buy_obstacle(
    _: On<BuyObstacleEvent>,
    mut commands: Commands,
    ressources_handler: Res<RessourcesHandler>,
    mut currency: ResMut<Currency>,
    mut obstacle_data: ResMut<ObstacleGlobalData>,
) {
    if currency.coin >= obstacle_data.price {
        currency.coin -= obstacle_data.price;
        obstacle_data.price += (obstacle_data.price as f32 * 0.5) as u32;
        commands.spawn(obstacle(&ressources_handler));
    }
}

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
