use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use bevy::ui_widgets::{Activate, observe};

use crate::scenes::AppState;
use crate::scenes::battlefield::BattleFieldSet;
use crate::scenes::battlefield::currency::Currency;
use crate::scenes::battlefield::map_validity::MapValidity;
use crate::scenes::battlefield::obstacle::{BuyObstacleEvent, ObstacleGlobalData};
use crate::scenes::battlefield::pathfinding::{
    PartialUpdatePathfindingMapEvent, UpdatePathfindingMapEvent,
};
use crate::scenes::battlefield::tower::{BuyTowerEvent, TowerGlobalData};
use crate::scenes::battlefield::wave::{LaunchWaveEvent, WavePhase};
use crate::ui::EnableButtonEvent;

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_buy_tower_button.run_if(resource_exists_and_changed::<Currency>),
                update_buy_obstacle_button.run_if(resource_exists_and_changed::<Currency>),
            )
                .in_set(BattleFieldSet),
        )
        .add_systems(OnEnter(WavePhase::Finished), enable_buttons)
        .add_systems(OnExit(WavePhase::Finished), disable_buttons);
    }
}

#[derive(Component)]
struct BuyTowerButton;

#[derive(Component)]
struct BuyObstacleButton;

#[derive(Component)]
struct ShopButton;

pub fn shop() -> impl Bundle {
    (
        Node {
            width: px(400),
            height: px(600),
            border_radius: BorderRadius::all(px(20)),
            align_self: AlignSelf::Center,
            margin: UiRect {
                left: px(10),
                top: px(10),
                bottom: px(10),
                ..UiRect::default()
            },

            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(10),
            ..default()
        },
        BackgroundColor(tailwind::INDIGO_900.into()),
        children![
            (
                BuyTowerButton,
                ShopButton,
                button("Buy tower"),
                observe(|_: On<Activate>, mut commands: Commands| {
                    commands.trigger(BuyTowerEvent);
                    commands.trigger(PartialUpdatePathfindingMapEvent);
                })
            ),
            (
                BuyObstacleButton,
                ShopButton,
                button("Buy obstacle"),
                observe(|_: On<Activate>, mut commands: Commands| {
                    commands.trigger(BuyObstacleEvent);
                    commands.trigger(PartialUpdatePathfindingMapEvent);
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
        **text = format!("Buy Tower\n{}", tower_data.price as u32);
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
        **text = format!("Buy Obstacle\n{}", obstacle_data.price as u32);
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

fn enable_buttons(mut commands: Commands, button: Query<Entity, With<ShopButton>>) {
    for entity in button {
        commands.trigger(EnableButtonEvent {
            entity,
            enable: true,
        });
    }
}

fn disable_buttons(mut commands: Commands, button: Query<Entity, With<ShopButton>>) {
    for entity in button {
        commands.trigger(EnableButtonEvent {
            entity,
            enable: false,
        });
    }
}
