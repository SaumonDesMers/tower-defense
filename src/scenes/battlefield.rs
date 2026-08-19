use avian2d::prelude::*;
use bevy::ui::prelude::*;
use bevy::{color::palettes::tailwind, prelude::*};

mod base;
mod buildings;
mod click_attack;
mod currency;
mod enemy;
mod health;
mod lifetime;
mod map_validity;
mod obstacle;
mod pathfinding;
mod physic;
mod projectile;
mod selection;
mod spawner;
mod tower;
mod ui;
mod wave;

use crate::scenes::battlefield::map_validity::{MapValidity, MapValidityPlugin};
use crate::scenes::battlefield::wave::WaveSpawnerZone;
use crate::scenes::{
    AppState,
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
        .add_plugins(MapValidityPlugin)
        .add_systems(OnEnter(AppState::InGame), setup)
        .add_systems(OnExit(AppState::InGame), cleanup)
        .configure_sets(Update, BattleFieldSet.run_if(in_state(AppState::InGame)));
    }
}

#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone)]
pub struct BattleFieldSet;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("Setting up battlefield...");
    // BackGround
    commands.spawn((
        DespawnOnExit(AppState::InGame),
        Transform::from_xyz(0.0, 0.0, -0.2),
        Mesh2d(meshes.add(Rectangle::new(10000.0, 10000.0))),
        MeshMaterial2d(materials.add(Color::from(tailwind::LIME_800))),
    ));

    // Ground
    commands.spawn((
        DespawnOnExit(AppState::InGame),
        Transform::from_xyz(0.0, 0.0, -0.1),
        Mesh2d(meshes.add(Rectangle::new(2000.0, 1000.0))),
        MeshMaterial2d(materials.add(Color::from(tailwind::ORANGE_300))),
    ));

    // Base
    commands.spawn((
        DespawnOnExit(AppState::InGame),
        Transform::from_translation(BASE_POSITION.extend(1.0)),
        Mesh2d(meshes.add(Circle::new(30.0))),
        MeshMaterial2d(materials.add(Color::from(tailwind::STONE_600))),
        Base { life: 50000 },
        Collider::circle(30.0),
    ));

    // Wave Spawner
    commands.spawn((
        DespawnOnExit(AppState::InGame),
        WaveSpawnerZone(Vec2::new(100.0, 1000.0)),
        Transform::from_xyz(950.0, 0.0, 0.0),
        Mesh2d(meshes.add(Rectangle::new(100.0, 1000.0))),
        MeshMaterial2d(materials.add(Color::from(tailwind::RED_600))),
        Collider::from(Rectangle::new(100.0, 1000.0)),
    ));

    // UI
    commands.spawn((DespawnOnExit(AppState::InGame), ui::ui()));

    commands.insert_resource(Currency { coin: 10000 });
    commands.insert_resource(MapValidity { error: None });
    commands.insert_resource(PathfindingMap::new(
        Vec2::new(-1000.0, -500.0),
        200,
        100,
        10.0,
    ));
    commands.insert_resource(WaveGlobalData {
        count: 1,
        delay: 0.5,
    });
    commands.insert_resource(TowerGlobalData { price: 10 });
    commands.insert_resource(ObstacleGlobalData { price: 10 });
    commands.insert_resource(ClickAttackGlobalData {
        damage: Damage::new(10),
        mesh: meshes.add(Circle::new(100.0)),
    });
}

fn cleanup(mut commands: Commands) {
    info!("Cleaning up battlefield...");
    commands.remove_resource::<Currency>();
    commands.remove_resource::<MapValidity>();
    commands.remove_resource::<PathfindingMap>();
    commands.remove_resource::<WaveGlobalData>();
    commands.remove_resource::<TowerGlobalData>();
    commands.remove_resource::<ObstacleGlobalData>();
    commands.remove_resource::<ClickAttackGlobalData>();
}
