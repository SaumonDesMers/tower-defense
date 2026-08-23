use avian2d::prelude::*;
use bevy::color::palettes::tailwind;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

use super::buildings::Building;
use super::physic::GameLayer;
use super::selection::Selectable;
use crate::RessourcesHandler;
use crate::scenes::AppState;
use crate::scenes::battlefield::currency::Coins;
use crate::scenes::battlefield::ui::{Price, ShopItem};

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(buy_obstacle.run_if(in_state(AppState::InGame)));
    }
}

const OBSTACLE_PRICE: f32 = 10.0;

pub struct Obstacle;

impl ShopItem for Obstacle {
    fn spawn(&self, commands: &mut RelatedSpawnerCommands<'_, ChildOf>) {
        commands.spawn((Text::new("Obstacle"), TextColor(tailwind::SLATE_200.into())));
    }

    fn price(&self) -> Price {
        Price(OBSTACLE_PRICE)
    }

    fn buy(&self, commands: &mut Commands) {
        commands.trigger(BuyObstacleEvent);
    }
}

#[derive(Event)]
pub struct BuyObstacleEvent;

fn buy_obstacle(
    _: On<BuyObstacleEvent>,
    mut commands: Commands,
    ressources_handler: Res<RessourcesHandler>,
    mut coins: ResMut<Coins>,
) {
    if coins.0 >= OBSTACLE_PRICE {
        coins.0 -= OBSTACLE_PRICE;
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
