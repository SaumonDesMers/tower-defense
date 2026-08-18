use avian2d::prelude::*;
use bevy::prelude::*;

use crate::scenes::{AppState, battlefield::BattleFieldSet};

pub const BASE_POSITION: Vec2 = Vec2::new(-900.0, 0.0);

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, base_system.in_set(BattleFieldSet));
    }
}

#[derive(Component)]
pub struct Base {
    pub life: u32,
}

fn base_system(query: Query<&Base>, mut scene_next_state: ResMut<NextState<AppState>>) {
    if let Ok(base) = query.single() {
        if base.life == 0 {
            scene_next_state.set(AppState::GameOver);
        }
    }
}
