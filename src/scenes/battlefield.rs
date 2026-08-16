use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::ui::prelude::*;

mod base;
mod buildings;
mod click_attack;
mod currency;
mod enemy;
mod health;
mod lifetime;
mod obstacle;
mod pathfinding;
mod physic;
mod projectile;
mod selection;
mod spawner;
mod tower;
mod ui;
mod wave;

use crate::scenes::{
    SceneState,
    battlefield::{
        click_attack::{ClickAttackGlobalData, ClickAttackPlugin},
        currency::{Currency, CurrencyPlugin},
        obstacle::{ObstacleGlobalData, ObstaclePlugin},
        pathfinding::PathfindingMap,
        tower::TowerGlobalData,
        wave::WaveGlobalData,
    },
};

use base::{BASE_POSITION, Base, BasePlugin};
use buildings::{Building, BuildingsPlugin};
use enemy::{Enemy, EnemyAI, EnemyPlugin};
use health::{Damage, Health, HealthPlugin};
use pathfinding::PathfindingPlugin;
use physic::GameLayer;
use projectile::{Projectile, ProjectileHitEvent, ProjectilePlugin};
use selection::{Selectable, SelectionPlugin};
use spawner::{Spawnable, Spawner, SpawnerPlugin};
use tower::{Tower, TowerPlugin, tower};
use wave::WavePlugin;

pub struct BattleFieldPlugin;

impl Plugin for BattleFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ui::UiPlugin,
            HealthPlugin,
            lifetime::LifetimePlugin,
            SpawnerPlugin,
            EnemyPlugin,
            ProjectilePlugin,
            PathfindingPlugin,
            BuildingsPlugin,
            WavePlugin,
            SelectionPlugin,
            TowerPlugin,
            BasePlugin,
            ObstaclePlugin,
            CurrencyPlugin,
            ClickAttackPlugin,
        ))
        .add_systems(OnEnter(SceneState::Battlefield), setup)
        .add_systems(OnExit(SceneState::Battlefield), cleanup)
        .configure_sets(
            Update,
            BattleFieldSet.run_if(in_state(SceneState::Battlefield)),
        );
    }
}

#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone)]
pub struct BattleFieldSet;

#[derive(Component, Clone)]
pub struct BattleField;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("Setting up battlefield...");
    // Background
    commands.spawn((
        BattleField,
        Transform::from_xyz(0.0, 0.0, -0.1),
        Mesh2d(meshes.add(Rectangle::new(2000.0, 1000.0))),
        MeshMaterial2d(materials.add(Color::srgba(0.5, 0.5, 0.5, 1.0))),
    ));

    // UI
    commands.spawn((BattleField, ui::ui()));

    commands.insert_resource(Currency { coin: 0 });
    commands.insert_resource(PathfindingMap::new(
        Vec2::new(-1000.0, -500.0),
        200,
        100,
        10.0,
    ));
    commands.insert_resource(WaveGlobalData::new());
    commands.insert_resource(TowerGlobalData { price: 10 });
    commands.insert_resource(ObstacleGlobalData { price: 10 });
    commands.insert_resource(ClickAttackGlobalData {
        damage: Damage::new(10),
        mesh: meshes.add(Circle::new(100.0)),
    });
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<BattleField>>) {
    info!("Cleaning up battlefield...");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    commands.remove_resource::<Currency>();
    commands.remove_resource::<PathfindingMap>();
    commands.remove_resource::<WaveGlobalData>();
    commands.remove_resource::<TowerGlobalData>();
    commands.remove_resource::<ObstacleGlobalData>();
    commands.remove_resource::<ClickAttackGlobalData>();
}
