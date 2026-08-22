use bevy::ui::prelude::*;
use bevy::ui_widgets::{Activate, observe};
use bevy::{color::palettes::tailwind, prelude::*};

use crate::scenes::AppState;
use crate::scenes::battlefield::attack_range::{AttackRange, AttackRangeType};
use crate::scenes::battlefield::attack_speed::{self, AttackSpeed};
use crate::scenes::battlefield::base::Base;
use crate::scenes::battlefield::currency::Coins;
use crate::scenes::battlefield::damage::Damage;
use crate::scenes::battlefield::health::Health;
use crate::scenes::battlefield::selection::{Selectable, Selection};
use crate::scenes::battlefield::tower::TowerGlobalData;
use crate::scenes::battlefield::ui::inspector;
use crate::scenes::battlefield::ui::next_wave::DisabledDuringWave;
use crate::ui::EnableButtonEvent;

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(update_inspector).add_systems(
            Update,
            update_upgrade_button.run_if(resource_exists_and_changed::<Coins>),
        );
    }
}

#[derive(Component)]
struct Inspector;

#[derive(Event)]
pub struct UpdateInspector;

#[derive(Component)]
struct UpgradeButton;

pub fn inspector_window() -> impl Bundle {
    (
        Node {
            width: px(400),
            height: px(600),
            border_radius: BorderRadius::all(px(20)),
            align_self: AlignSelf::Center,
            margin: UiRect {
                right: px(10),
                top: px(10),
                bottom: px(10),
                ..UiRect::default()
            },

            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(50),
            ..default()
        },
        BackgroundColor(tailwind::INDIGO_900.into()),
        Inspector,
        Visibility::Inherited,
        children![
            (
                Text::new("Inspector"),
                TextColor(tailwind::SLATE_200.into()),
            ),
            (
                Button,
                DisabledDuringWave,
                UpgradeButton,
                Node {
                    // width: px(200),
                    margin: UiRect {
                        right: px(10),
                        top: px(10),
                        ..UiRect::default()
                    },
                    border_radius: BorderRadius::all(px(10)),

                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect {
                        left: px(15),
                        right: px(15),
                        top: px(15),
                        bottom: px(15),
                    },
                    ..default()
                },
                children![(Text::new("Upgrade"), TextColor(tailwind::SLATE_200.into()),)],
                observe(|_: On<Activate>| {
                    info!("Upgrade !");
                },),
            )
        ],
    )
}

fn update_inspector(
    _: On<UpdateInspector>,
    mut inspector: Query<(&mut Visibility, &Children), With<Inspector>>,
    mut text: Query<&mut Text>,
    selectables: Query<
        (
            &Name,
            Option<&Health>,
            Option<&AttackSpeed>,
            Option<&Damage>,
            Option<&AttackRange>,
        ),
        With<Selectable>,
    >,
    selection: Res<Selection>,
) {
    let (mut visibility, children) = inspector.single_mut().expect("Inpector should exist");

    let Some(selected) = selection.entity else {
        *visibility = Visibility::Hidden;
        return;
    };

    let mut text = text
        .get_mut(
            *children
                .get(0)
                .expect("Inspector should have at least one child"),
        )
        .expect("Inpector text should exist");

    *visibility = Visibility::Inherited;

    let (name, health, attack_speed, damage, range) = selectables
        .get(selected)
        .expect("Selected entity should exist");

    **text = name.to_string() + "\n";
    if let Some(health) = health {
        text.push_str(&format!(
            "\nHealth: {:.2}/{:.2}",
            health.current, health.max
        ));
    }
    if let Some(attack_speed) = attack_speed {
        text.push_str(&format!("\nAttack speed: {:.2}", attack_speed.per_second()));
    }
    if let Some(damage) = damage {
        text.push_str(&format!("\nDamage: {:.2}", damage.amount));
    }
    if let Some(range) = range {
        text.push_str(&match range.range_type {
            AttackRangeType::Circle(radius) => format!("\nRange radius: {}", radius.ceil()),
        });
    }
}

fn update_upgrade_button(
    mut commands: Commands,
    mut button: Query<(Entity, &Children), With<UpgradeButton>>,
    mut texts: Query<&mut Text>,
    coins: Res<Coins>,
    tower_data: Res<TowerGlobalData>,
    mut had_enough_coin: Local<Option<bool>>,
) {
    if let Ok((entity, children)) = button.single_mut() {
        let mut text = texts
            .get_mut(children[0])
            .expect("Should have child with Text.");
        **text = format!("Upgrade ({} coins)", tower_data.upgrade_price as u32);
        let has_enough_coin = coins.0 >= tower_data.upgrade_price;
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
