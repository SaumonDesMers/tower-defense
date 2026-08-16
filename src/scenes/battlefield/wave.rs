use avian2d::prelude::*;
use bevy::prelude::*;

use super::BattleField;
use super::enemy::{Enemy, EnemyAI};
use super::health::Health;
use super::lifetime::Lifetime;
use super::spawner::{Spawnable, Spawner};

use crate::RessourcesHandler;
use crate::scenes::battlefield::currency::DropCoin;
use crate::scenes::battlefield::wave;

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(start_wave)
            .add_observer(detect_finish_spawning.run_if(in_state(WaveState::Spawning)))
            .add_systems(
                Update,
                detect_finish_killing.run_if(in_state(WaveState::Killing)),
            )
            .init_state::<WaveState>();
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum WaveState {
    Spawning,
    Killing,
    #[default]
    Finished,
}

#[derive(Resource)]
pub struct WaveGlobalData {
    count: u32,
    delay: f32,
}

impl WaveGlobalData {
    pub fn new() -> Self {
        Self {
            count: 1,
            delay: 1.0,
        }
    }
}

#[derive(Event)]
pub struct LaunchWaveEvent;

#[derive(Component)]
struct WaveSpawner;

fn start_wave(
    _: On<LaunchWaveEvent>,
    mut commands: Commands,
    ressources_handler: Res<RessourcesHandler>,
    mut next_wave_state: ResMut<NextState<WaveState>>,
    mut wave_data: ResMut<WaveGlobalData>,
) {
    let enemy_count = wave_data.count * wave_data.count;
    let total_time = (enemy_count as f32 + 0.1) * wave_data.delay;
    info!(
        "start wave with {enemy_count} enemies. One each {}s for {total_time}s.",
        wave_data.delay
    );

    next_wave_state.set(WaveState::Spawning);
    commands.spawn((
        BattleField,
        Transform::from_xyz(900.0, 0.0, 1.0),
        Spawner::new(
            std::time::Duration::from_secs_f32(wave_data.delay),
            Vec2::new(100.0, 1000.0),
            Spawnable((
                BattleField,
                Mesh2d(ressources_handler.enemy_mesh.clone()),
                MeshMaterial2d(ressources_handler.enemy_material.clone()),
                Collider::circle(5.0),
                RigidBody::Dynamic,
                Enemy,
                EnemyAI { speed: 100.0 },
                Health::new(5),
                DropCoin(1),
            )),
        ),
        Lifetime::new(std::time::Duration::from_secs_f32(total_time)),
        WaveSpawner,
    ));

    wave_data.count += 1;
    wave_data.delay *= 0.9;
}

fn detect_finish_spawning(
    _: On<Despawn, WaveSpawner>,
    mut next_wave_state: ResMut<NextState<WaveState>>,
) {
    next_wave_state.set(WaveState::Killing);
}

fn detect_finish_killing(
    enemies: Query<(), With<Enemy>>,
    mut next_wave_state: ResMut<NextState<WaveState>>,
) {
    if enemies.is_empty() {
        next_wave_state.set(WaveState::Finished);
    }
}
