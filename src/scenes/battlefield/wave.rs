use avian2d::prelude::*;
use bevy::prelude::*;

use super::enemy::{Enemy, EnemyAI};
use super::health::Health;
use super::lifetime::Lifetime;
use super::spawner::{Spawnable, Spawner};

use crate::RessourcesHandler;
use crate::scenes::AppState;
use crate::scenes::battlefield::currency::DropCoins;

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(start_wave)
            .add_observer(detect_finish_spawning.run_if(in_state(WavePhase::Spawning)))
            .add_systems(
                Update,
                detect_finish_killing.run_if(in_state(WavePhase::Killing)),
            )
            .add_sub_state::<WavePhase>();
    }
}

#[derive(SubStates, Debug, Clone, Eq, PartialEq, Hash, Default)]
#[source(AppState = AppState::InGame)]
pub enum WavePhase {
    Spawning,
    Killing,
    #[default]
    Finished,
}

#[derive(Resource)]
pub struct WaveGlobalData {
    pub count: u32,
    pub delay: f32,
}

#[derive(Event)]
pub struct LaunchWaveEvent;

#[derive(Component)]
pub struct WaveSpawnerZone(pub Vec2);

#[derive(Component)]
struct WaveSpawner;

fn start_wave(
    _: On<LaunchWaveEvent>,
    mut commands: Commands,
    wave_spawner: Query<(Entity, &WaveSpawnerZone)>,
    ressources_handler: Res<RessourcesHandler>,
    mut next_wave_state: ResMut<NextState<WavePhase>>,
    mut wave_data: ResMut<WaveGlobalData>,
) {
    let Ok((spawner, zone)) = wave_spawner.single() else {
        warn!("No wave spawner found");
        return;
    };

    let enemy_count = wave_data.count * wave_data.count + 10;
    let total_time = (enemy_count as f32 + 0.1) * wave_data.delay;
    info!(
        "start wave with {enemy_count} enemies. One each {}s for {total_time}s.",
        wave_data.delay
    );

    next_wave_state.set(WavePhase::Spawning);
    commands.spawn((
        WaveSpawner,
        Transform::from_xyz(0.0, 0.0, 1.0),
        Spawner::new(
            std::time::Duration::from_secs_f32(wave_data.delay),
            zone.0,
            Spawnable((
                DespawnOnExit(AppState::InGame),
                Mesh2d(ressources_handler.enemy_mesh.clone()),
                MeshMaterial2d(ressources_handler.enemy_material.clone()),
                Collider::circle(5.0),
                RigidBody::Dynamic,
                Enemy,
                EnemyAI { speed: 100.0 },
                Health::new(5.0),
                DropCoins(1.0),
            )),
        ),
        Lifetime::new(std::time::Duration::from_secs_f32(total_time)),
        ChildOf(spawner),
    ));

    wave_data.count += 1;
    wave_data.delay *= 0.9;
}

fn detect_finish_spawning(
    _: On<Despawn, WaveSpawner>,
    mut next_wave_state: ResMut<NextState<WavePhase>>,
) {
    next_wave_state.set(WavePhase::Killing);
}

fn detect_finish_killing(
    enemies: Query<(), With<Enemy>>,
    mut next_wave_state: ResMut<NextState<WavePhase>>,
) {
    if enemies.is_empty() {
        next_wave_state.set(WavePhase::Finished);
    }
}
