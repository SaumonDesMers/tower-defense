use bevy::prelude::*;
use bevy::ui::prelude::*;
use avian2d::prelude::*;

mod ui;
mod health;
mod lifetime;
mod spawner;
mod enemy;
mod projectile;
mod pathfinding;
mod physic;
mod buildings;
mod wave;
mod selection;
mod tower;
mod obstacle;
mod base;

use crate::scenes::SceneState;

use health::{Health, HealthPlugin, Damage};
use spawner::{Spawnable, Spawner, SpawnerPlugin};
use enemy::{Enemy, EnemyAI, EnemyPlugin};
use projectile::{ProjectilePlugin, Projectile, ProjectileHitEvent};
use pathfinding::{PathfindingPlugin};
use physic::GameLayer;
use buildings::{BuildingsPlugin, Building};
use wave::WavePlugin;
use selection::{SelectionPlugin, Selectable};
use tower::{TowerPlugin, Tower, tower};
use base::{BasePlugin, Base, BASE_POSITION};

pub struct BattleFieldPlugin;

impl Plugin for BattleFieldPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugins((
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
			))
			.add_systems(OnEnter(SceneState::Game), setup_battlefield)
			.add_systems(OnExit(SceneState::Game), cleanup_battlefield);
	}
}

#[derive(Component, Clone)]
pub struct BattleField;

fn setup_battlefield(
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

	// Base
	commands.spawn((
		BattleField,
		Transform::from_translation(BASE_POSITION.extend(1.0)),
		Mesh2d(meshes.add(Circle::new(10.0))),
		MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
		Base { life: 5 },
		Collider::circle(10.0),
	));

	// UI
	commands.spawn((
		BattleField,
		ui::ui(),
	));
}

fn cleanup_battlefield(
	mut commands: Commands,
	query: Query<Entity, With<BattleField>>,
) {
	info!("Cleaning up battlefield...");
	for entity in query.iter() {
		commands.entity(entity).despawn();
	}
}
