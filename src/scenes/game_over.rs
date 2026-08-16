use bevy::color::palettes::tailwind;
use bevy::input_focus::InputFocus;
use bevy::log::tracing_subscriber::field::debug;
use bevy::prelude::*;
use bevy::ui::prelude::*;
use bevy::ui_widgets::{Activate, observe};

use crate::scenes::SceneState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::GameOver), setup)
            .add_systems(OnExit(SceneState::GameOver), cleanup);
    }
}

#[derive(Component)]
struct GameOver;

fn setup(mut commands: Commands) {
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
                button("Restart"),
                observe(
                    |_activate: On<Activate>,
                     mut scene_next_state: ResMut<NextState<SceneState>>| {
                        scene_next_state.set(SceneState::Battlefield);
                    }
                )
            ),
            (
                button("Main menu"),
                observe(
                    |_activate: On<Activate>,
                     mut scene_next_state: ResMut<NextState<SceneState>>| {
                        scene_next_state.set(SceneState::MainMenu);
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
