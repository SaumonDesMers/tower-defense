use avian2d::prelude::*;
use bevy::prelude::*;

use crate::scenes::{
    SceneState,
    battlefield::{BattleField, BattleFieldSet},
};

pub const BASE_POSITION: Vec2 = Vec2::new(-900.0, 0.0);

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::Battlefield), setup)
            .add_systems(Update, base_system.in_set(BattleFieldSet));
    }
}

#[derive(Component)]
pub struct Base {
    pub life: u32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        BattleField,
        Transform::from_translation(BASE_POSITION.extend(1.0)),
        Mesh2d(meshes.add(Circle::new(10.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
        Base { life: 50000 },
        Collider::circle(10.0),
    ));
}

fn base_system(query: Query<&Base>, mut scene_next_state: ResMut<NextState<SceneState>>) {
    if let Ok(base) = query.single() {
        if base.life == 0 {
            scene_next_state.set(SceneState::GameOver);
        }
    }
}
