use bevy::color::palettes::tailwind;
use bevy::ecs::system::command;
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
use crate::scenes::battlefield::currency::{self, Currency};
use crate::scenes::battlefield::obstacle::{BuyObstacleEvent, ObstacleGlobalData};
use crate::scenes::battlefield::tower::{BuyTowerEvent, TowerGlobalData};
use crate::scenes::battlefield::wave::WaveState;
use crate::ui::ButtonDisabled;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_currency_display.run_if(resource_exists_and_changed::<Currency>),
                update_buy_tower_button.run_if(resource_exists_and_changed::<Currency>),
                update_buy_obstacle_button.run_if(resource_exists_and_changed::<Currency>),
            ),
        )
        .add_systems(OnEnter(WaveState::Finished), enable_buttons_after_wave)
        .add_systems(OnEnter(WaveState::Spawning), disable_buttons_during_wave);
    }
}

#[derive(Component)]
struct CurrencyDisplay;

#[derive(Component)]
struct BuyTowerButton;

#[derive(Component)]
struct BuyObstacleButton;

#[derive(Component)]
struct NextWaveButton;

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
        BackgroundColor(tailwind::INDIGO_900.into()),
        children![
            (
                button("Main menu"),
                observe(
                    |_: On<Activate>, mut scene_next_state: ResMut<NextState<SceneState>>| {
                        scene_next_state.set(SceneState::MainMenu);
                    }
                )
            ),
            (
                CurrencyDisplay,
                Text::new("placeholder"),
                TextColor(tailwind::SLATE_200.into()),
            ),
            (
                BuyTowerButton,
                button("Buy tower"),
                observe(|_: On<Activate>, mut commands: Commands| {
                    commands.trigger(BuyTowerEvent);
                })
            ),
            (
                BuyObstacleButton,
                button("Buy obstacle"),
                observe(|_: On<Activate>, mut commands: Commands| {
                    commands.trigger(BuyObstacleEvent);
                })
            ),
            (
                NextWaveButton,
                button("Next Wave"),
                observe(|_: On<Activate>, mut commands: Commands| {
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

fn update_currency_display(
    currency: Res<Currency>,
    mut display: Query<&mut Text, With<CurrencyDisplay>>,
) {
    if let Ok(mut text) = display.single_mut() {
        **text = format!("Coin: {}", currency.coin);
    }
}

fn update_buy_tower_button(
    mut commands: Commands,
    mut button: Query<(Entity, &Children), With<BuyTowerButton>>,
    mut texts: Query<&mut Text>,
    currency: Res<Currency>,
    tower_global_data: Res<TowerGlobalData>,
) {
    if let Ok((entity, children)) = button.single_mut() {
        let mut text = texts
            .get_mut(children[0])
            .expect("Should have child with Text.");
        **text = format!("Buy Tower\n{}", tower_global_data.price);
        if currency.coin < tower_global_data.price {
            commands.entity(entity).insert(ButtonDisabled);
        } else {
            commands.entity(entity).remove::<ButtonDisabled>();
        }
    }
}

fn update_buy_obstacle_button(
    mut commands: Commands,
    mut button: Query<(Entity, &Children), With<BuyObstacleButton>>,
    mut texts: Query<&mut Text>,
    currency: Res<Currency>,
    obstacle_data: Res<ObstacleGlobalData>,
) {
    if let Ok((entity, children)) = button.single_mut() {
        let mut text = texts
            .get_mut(children[0])
            .expect("Should have child with Text.");
        **text = format!("Buy Obstacle\n{}", obstacle_data.price);
        if currency.coin < obstacle_data.price {
            commands.entity(entity).insert(ButtonDisabled);
        } else {
            commands.entity(entity).remove::<ButtonDisabled>();
        }
    }
}

fn disable_buttons_during_wave(
    mut commands: Commands,
    buttons: Query<
        Entity,
        Or<(
            With<BuyTowerButton>,
            With<BuyObstacleButton>,
            With<NextWaveButton>,
        )>,
    >,
) {
    for entity in buttons {
        commands.entity(entity).insert(ButtonDisabled);
    }
}

fn enable_buttons_after_wave(
    mut commands: Commands,
    buttons: Query<
        Entity,
        Or<(
            With<BuyTowerButton>,
            With<BuyObstacleButton>,
            With<NextWaveButton>,
        )>,
    >,
) {
    for entity in buttons {
        commands.entity(entity).remove::<ButtonDisabled>();
    }
}
