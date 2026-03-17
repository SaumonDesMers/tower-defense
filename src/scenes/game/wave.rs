use bevy::prelude::*;
use avian2d::prelude::*;

use super::spawner::{Spawnable, Spawner};
use super::enemy::{Enemy, EnemyAI};
use super::health::Health;
use super::lifetime::Lifetime;
use super::BattleField;

use crate::RessourcesHandler;

pub struct WavePlugin;

impl Plugin for WavePlugin {
	fn build(&self, app: &mut App) {
		app.add_observer(start_wave);
	}
}

#[derive(Event)]
pub struct LaunchWaveEvent;

fn start_wave(
	_start_wave_event: On<LaunchWaveEvent>,
	mut commands: Commands,
	ressources_handler: Res<RessourcesHandler>,
) {
	commands.spawn((
        BattleField,
        Transform::from_xyz(400.0, 0.0, 1.0),
        Spawner::new(
            std::time::Duration::from_millis(500),
            Vec2::new(200.0, 600.0),
            Spawnable((
                BattleField,
                Mesh2d(ressources_handler.enemy_mesh.clone()),
                MeshMaterial2d(ressources_handler.enemy_material.clone()),
                Collider::circle(5.0),
                RigidBody::Dynamic,
                Enemy,
                EnemyAI { speed: 100.0 },
                Health::new(5),
            )),
        ),
		Lifetime::new(std::time::Duration::from_secs(10)),
    ));
}