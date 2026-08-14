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
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_system);
    }
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
        BackgroundColor(tailwind::ORANGE_200.into()),
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

const NORMAL_BUTTON: Srgba = tailwind::ORANGE_300;
const HOVERED_BUTTON: Srgba = tailwind::ORANGE_400;
const PRESSED_BUTTON: Srgba = tailwind::ORANGE_500;

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Button,
        ),
        Changed<Interaction>,
    >,
) {
    for (interaction, mut color, mut border_color, mut button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor::from(PRESSED_BUTTON);
                *border_color = BorderColor::all(tailwind::RED_400);
                button.set_changed();
            }
            Interaction::Hovered => {
                *color = BackgroundColor::from(HOVERED_BUTTON);
                *border_color = BorderColor::all(tailwind::ORANGE_500);
                button.set_changed();
            }
            Interaction::None => {
                *color = BackgroundColor::from(NORMAL_BUTTON);
                *border_color = BorderColor::all(tailwind::ORANGE_400);
            }
        }
    }
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
            border_radius: BorderRadius::all(percent(20)),
            ..default()
        },
        BackgroundColor::from(NORMAL_BUTTON),
        BorderColor::all(tailwind::ORANGE_400),
        children![(Text::new(text), TextColor(tailwind::ORANGE_100.into()),)],
    )
}
