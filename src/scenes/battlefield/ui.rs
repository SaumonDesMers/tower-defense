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
use crate::scenes::AppState;
use crate::scenes::battlefield::BattleFieldSet;
use crate::scenes::battlefield::currency::{self, Currency};
use crate::scenes::battlefield::map_validity::MapValidity;
use crate::scenes::battlefield::obstacle::{BuyObstacleEvent, ObstacleGlobalData};
use crate::scenes::battlefield::tower::{BuyTowerEvent, TowerGlobalData};
use crate::scenes::battlefield::wave::WavePhase;
use crate::ui::EnableButtonEvent;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_currency_display.run_if(resource_exists_and_changed::<Currency>),
                update_map_validity_display.run_if(resource_exists_and_changed::<MapValidity>),
                update_next_wave_button.run_if(resource_exists_and_changed::<MapValidity>),
                update_buy_tower_button.run_if(resource_exists_and_changed::<Currency>),
                update_buy_obstacle_button.run_if(resource_exists_and_changed::<Currency>),
            )
                .in_set(BattleFieldSet),
        )
        .add_systems(
            OnTransition {
                exited: WavePhase::Killing,
                entered: WavePhase::Finished,
            },
            enable_buttons_after_wave,
        )
        .add_systems(OnEnter(WavePhase::Spawning), disable_buttons_during_wave);
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

#[derive(Component)]
struct MapValidityDisplay;

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
                    |_: On<Activate>, mut scene_next_state: ResMut<NextState<AppState>>| {
                        scene_next_state.set(AppState::MainMenu);
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
                    commands.trigger(UpdatePathfindingMapEvent);
                })
            ),
            (
                BuyObstacleButton,
                button("Buy obstacle"),
                observe(|_: On<Activate>, mut commands: Commands| {
                    commands.trigger(BuyObstacleEvent);
                    commands.trigger(UpdatePathfindingMapEvent);
                })
            ),
            (
                MapValidityDisplay,
                Text::new("placeholder"),
                TextColor(tailwind::SLATE_200.into()),
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
    tower_data: Res<TowerGlobalData>,
    mut had_enough_coin: Local<Option<bool>>,
) {
    if let Ok((entity, children)) = button.single_mut() {
        let mut text = texts
            .get_mut(children[0])
            .expect("Should have child with Text.");
        **text = format!("Buy Tower\n{}", tower_data.price);
        let has_enough_coin = currency.coin >= tower_data.price;
        if *&had_enough_coin.is_none()
            || (*had_enough_coin).expect("Should not execute if None because of previous condition")
                != has_enough_coin
        {
            *had_enough_coin = Some(has_enough_coin);
            commands.trigger(EnableButtonEvent {
                entity,
                enable: has_enough_coin,
            });
        }
    }
}

fn update_buy_obstacle_button(
    mut commands: Commands,
    mut button: Query<(Entity, &Children), With<BuyObstacleButton>>,
    mut texts: Query<&mut Text>,
    currency: Res<Currency>,
    obstacle_data: Res<ObstacleGlobalData>,
    mut had_enough_coin: Local<Option<bool>>,
) {
    if let Ok((entity, children)) = button.single_mut() {
        let mut text = texts
            .get_mut(children[0])
            .expect("Should have child with Text.");
        **text = format!("Buy Obstacle\n{}", obstacle_data.price);
        let has_enough_coin = currency.coin >= obstacle_data.price;
        if *&had_enough_coin.is_none()
            || (*had_enough_coin).expect("Should not execute if None because of previous condition")
                != has_enough_coin
        {
            *had_enough_coin = Some(has_enough_coin);
            commands.trigger(EnableButtonEvent {
                entity,
                enable: has_enough_coin,
            });
        }
    }
}

fn update_map_validity_display(
    map_validity: Res<MapValidity>,
    mut display: Query<&mut Text, With<MapValidityDisplay>>,
) {
    if let Ok(mut text) = display.single_mut() {
        **text = match &map_validity.error {
            Some(msg) => format!("{}", msg),
            None => format!("Map Valid"),
        };
    }
}

fn update_next_wave_button(
    mut commands: Commands,
    mut button: Query<Entity, With<NextWaveButton>>,
    map_validity: Res<MapValidity>,
    mut map_was_valid: Local<Option<bool>>,
) {
    if let Ok(entity) = button.single_mut() {
        let map_is_valid = map_validity.error.is_none();
        if *&map_was_valid.is_none()
            || (*map_was_valid).expect("Should not execute if None because of previous condition")
                != map_is_valid
        {
            *map_was_valid = Some(map_is_valid);
            commands.trigger(EnableButtonEvent {
                entity,
                enable: map_is_valid,
            });
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
        commands.trigger(EnableButtonEvent {
            entity,
            enable: false,
        });
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
        commands.trigger(EnableButtonEvent {
            entity,
            enable: true,
        });
    }
}
