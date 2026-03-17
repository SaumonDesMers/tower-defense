use bevy::prelude::*;

mod main_menu;
mod game;
mod game_over;

pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugins((
				main_menu::MainMenuPlugin,
				game::BattleFieldPlugin,
				game_over::GameOverPlugin,
			))
			.init_state::<SceneState>();
	}
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum SceneState {
	#[default]
	Menu,
	Game,
	GameOver,
}