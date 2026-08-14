use avian2d::prelude::*;
use bevy::prelude::*;

use crate::scenes::SceneState;

pub const BASE_POSITION: Vec2 = Vec2::new(-900.0, 0.0);

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::Game), setup)
            .add_systems(Update, base_system);
    }
}

#[derive(Component)]
pub struct Base {
    pub life: u32,
}

fn setup(
    mut _commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<ColorMaterial>>,
) {
}

fn base_system(query: Query<&Base>, mut scene_next_state: ResMut<NextState<SceneState>>) {
    if let Ok(base) = query.single() {
        if base.life == 0 {
            scene_next_state.set(SceneState::GameOver);
        }
    }
}
