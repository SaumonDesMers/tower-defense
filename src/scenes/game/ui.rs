use bevy::color::palettes::tailwind;
use bevy::input_focus::InputFocus;
use bevy::log::tracing_subscriber::field::debug;
use bevy::prelude::*;
use bevy::ui::prelude::*;
use bevy::ui_widgets::{Activate, observe};

use super::obstacle::obstacle;
use super::pathfinding::UpdatePathfindingMapEvent;
use super::tower::tower;
use super::wave::LaunchWaveEvent;
use crate::RessourcesHandler;
use crate::scenes::SceneState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, _app: &mut App) {}
}

pub fn ui() -> impl Bundle {
    (
        Node {
            width: px(200),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: px(10),
            ..default()
        },
        BackgroundColor(tailwind::VIOLET_200.into()),
        children![
            (
                button("Main menu"),
                observe(
                    |_activate: On<Activate>,
                     mut scene_next_state: ResMut<NextState<SceneState>>| {
                        scene_next_state.set(SceneState::Menu);
                    }
                )
            ),
            (
                button("Buy tower"),
                observe(
                    |_activate: On<Activate>,
                     mut commands: Commands,
                     ressources_handler: Res<RessourcesHandler>| {
                        commands.spawn(tower(&ressources_handler));
                    }
                )
            ),
            (
                button("Buy obstacle"),
                observe(
                    |_activate: On<Activate>,
                     mut commands: Commands,
                     ressources_handler: Res<RessourcesHandler>| {
                        commands.spawn(obstacle(&ressources_handler));
                    }
                )
            ),
            (
                button("Next Wave"),
                observe(|_activate: On<Activate>, mut commands: Commands| {
                    commands.trigger(UpdatePathfindingMapEvent);
                    commands.trigger(LaunchWaveEvent);
                })
            ),
        ],
    )
}

fn button(text: &str) -> impl Bundle {
    (
        Button,
        Node {
            width: percent(80),
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
