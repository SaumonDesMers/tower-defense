use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use bevy::ui_widgets::{Activate, observe};

use crate::Score;
use crate::scenes::AppState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), setup)
            .add_systems(OnExit(AppState::GameOver), cleanup);
    }
}

#[derive(Component)]
struct GameOver;

fn setup(mut commands: Commands, mut score: ResMut<Score>) {
    info!("Setting up game over screen...");
    commands.spawn((
        GameOver,
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: px(10),
            ..default()
        },
        BackgroundColor(tailwind::INDIGO_900.into()),
        children![
            (
                Text::new("Game Over"),
                TextColor(tailwind::SLATE_200.into()),
            ),
            (
                Text::new({
                    if score.current > score.best {
                        score.best = score.current;
                    }
                    format!("Score: {}   (best: {})", score.current, score.best)
                }),
                TextColor(tailwind::SLATE_200.into()),
            ),
            (
                button("Restart"),
                observe(
                    |_activate: On<Activate>, mut scene_next_state: ResMut<NextState<AppState>>| {
                        scene_next_state.set(AppState::InGame);
                    }
                )
            ),
            (
                button("Main menu"),
                observe(
                    |_activate: On<Activate>, mut scene_next_state: ResMut<NextState<AppState>>| {
                        scene_next_state.set(AppState::MainMenu);
                    }
                )
            ),
        ],
    ));
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<GameOver>>) {
    info!("Cleaning up game over screen...");
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn button(text: &str) -> impl Bundle {
    (
        Button,
        Node {
            width: px(300),
            height: px(50),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: px(10),
            border: UiRect::all(px(5)),
            border_radius: BorderRadius::all(percent(10)),
            ..default()
        },
        children![(Text::new(text), TextColor(tailwind::SLATE_200.into()),)],
    )
}
