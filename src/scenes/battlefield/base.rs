use avian2d::prelude::*;
use bevy::prelude::*;

use crate::scenes::{
    AppState,
    battlefield::{BattleFieldSet, health::KilledEvent},
};

pub const BASE_POSITION: Vec2 = Vec2::new(-900.0, 0.0);

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(base_system.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
pub struct Base;

fn base_system(
    event: On<KilledEvent>,
    query: Query<(), With<Base>>,
    mut scene_next_state: ResMut<NextState<AppState>>,
) {
    if query.contains(event.entity) {
        scene_next_state.set(AppState::GameOver);
    }
}
