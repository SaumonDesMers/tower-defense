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
use crate::scenes::battlefield::relics::{AddRelic, Relic, Relics, RemoveRelic};
use crate::scenes::battlefield::selection::{Selectable, Selection};
use crate::scenes::battlefield::ui::inspector;
use crate::scenes::battlefield::ui::movable_item::{DropItemInSlot, MovableItem, MovableItemSlot};
use crate::scenes::battlefield::ui::next_wave::DisabledDuringWave;
use crate::scenes::battlefield::upgrade::{Level, UpgradeEvent};
use crate::ui::EnableButtonEvent;

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(update_inspector).add_systems(
            Update,
            update_upgrade_button.run_if(
                resource_exists_and_changed::<Coins>
                    .or_else(resource_exists_and_changed::<Selection>),
            ),
        );
    }
}

#[derive(Component)]
struct Inspector;

#[derive(Event)]
pub struct UpdateInspector;

#[derive(Component)]
struct UpgradeButton;

#[derive(Component)]
struct RelicSlots;

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
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: px(10),
                    ..default()
                },
                RelicSlots,
            ),
            (
                Button,
                DisabledDuringWave,
                UpgradeButton,
                Node {
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
                observe(on_activate_upgrade_button),
            )
        ],
    )
}

fn update_inspector(
    _: On<UpdateInspector>,
    mut commands: Commands,
    mut inspector: Query<(&mut Visibility, &Children), With<Inspector>>,
    mut upgrade_button: Single<&mut Node, With<UpgradeButton>>,
    relic_slots: Single<Entity, With<RelicSlots>>,
    mut text: Query<&mut Text>,
    selectables: Query<
        (
            &Name,
            Option<&Level>,
            Option<&Health>,
            Option<&AttackSpeed>,
            Option<&Damage>,
            Option<&AttackRange>,
            Option<&Relics>,
        ),
        With<Selectable>,
    >,
    selection: Res<Selection>,
) {
    info!("update inspector");
    let (mut visibility, children) = inspector.single_mut().expect("Inpector should exist");

    let Some(selected) = selection.entity else {
        *visibility = Visibility::Hidden;
        return;
    };
    *visibility = Visibility::Inherited;

    let mut text = text
        .get_mut(
            *children
                .get(0)
                .expect("Inspector should have at least one child"),
        )
        .expect("Inpector text should exist");

    let (name, level, health, attack_speed, damage, range, relics) = selectables
        .get(selected)
        .expect("Selected entity should exist");

    **text = name.to_string() + "\n";
    if let Some(level) = level {
        text.push_str(&format!("\nLevel: {}", level.count));
        upgrade_button.display = Display::Flex;
    } else {
        upgrade_button.display = Display::None;
    }
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

    commands
        .entity(*relic_slots)
        .despawn_children()
        .with_children(|slots| {
            let Some(relics) = relics else {
                return;
            };
            for relic in (0..relics.slot_count).map(|index| relics.list.get(index)) {
                slots
                    .spawn((
                        Node {
                            width: px(80),
                            height: px(80),
                            border_radius: BorderRadius::all(px(10)),
                            ..default()
                        },
                        MovableItemSlot,
                        observe(
                            |event: On<DropItemInSlot>,
                             mut commands: Commands,
                             relics: Query<&Relic>,
                             selection: Res<Selection>| {
                                if let Ok(relic) = relics.get(event.dropped) {
                                    commands.trigger(AddRelic {
                                        relic: relic.clone(),
                                        target: selection.entity.unwrap(),
                                    });
                                }
                            },
                        ),
                        BackgroundColor(tailwind::INDIGO_950.into()),
                    ))
                    .with_children(|slot| {
                        if let Some(relic) = relic {
                            slot.spawn((
                                MovableItem,
                                Node {
                                    width: px(80),
                                    height: px(80),
                                    border_radius: BorderRadius::all(px(10)),
                                    ..default()
                                },
                                BackgroundColor(tailwind::RED_600.into()),
                                relic.clone(),
                                children![(
                                    Text::new(relic.0.name()),
                                    TextColor(tailwind::SLATE_200.into()),
                                    Pickable::IGNORE,
                                )],
                            ));
                        }
                    });
            }
        });
}

#[derive(Component)]
pub struct HadEnoughCoin(bool);

fn update_upgrade_button(
    mut commands: Commands,
    button: Single<(Entity, Option<&mut HadEnoughCoin>, &Children), With<UpgradeButton>>,
    mut texts: Query<(&mut Text, &mut TextColor)>,
    level: Query<&Level>,
    selection: Res<Selection>,
    coins: Res<Coins>,
) {
    let (entity, maybe_had_enough_coin, children) = button.into_inner();
    let Some(selected_entity) = selection.entity else {
        return;
    };
    let Ok(level) = level.get(selected_entity) else {
        return;
    };
    let has_enough_coin = coins.0 >= level.price;

    let (mut text, mut color) = texts
        .get_mut(children[0])
        .expect("Should have child with Text.");

    **text = format!("Upgrade ({} coins)", level.price as u32);
    if has_enough_coin {
        *color = TextColor(tailwind::SLATE_200.into());
    } else {
        *color = TextColor(tailwind::RED_700.into())
    }

    if let Some(mut had_enough_coin) = maybe_had_enough_coin {
        if had_enough_coin.0 != has_enough_coin {
            had_enough_coin.0 = has_enough_coin;
            commands.trigger(EnableButtonEvent {
                entity,
                enable: has_enough_coin,
            });
        }
    } else {
        commands
            .entity(entity)
            .insert(HadEnoughCoin(has_enough_coin));
        if !has_enough_coin {
            commands.trigger(EnableButtonEvent {
                entity,
                enable: false,
            });
        }
    }
}

fn on_activate_upgrade_button(
    _: On<Activate>,
    mut commands: Commands,
    mut current_level: Query<&mut Level>,
    selection: Res<Selection>,
    mut coins: ResMut<Coins>,
) {
    let selected_entity = selection.entity.expect("Selected entity should exist");
    let mut level = current_level
        .get_mut(selected_entity)
        .expect("Selected entity should have Level Component if upgrade button is triggered");
    if coins.0 >= level.price {
        coins.0 -= level.price;
        level.count += 1;
        level.price *= level.price_mul;
        commands.trigger(UpgradeEvent {
            entity: selected_entity,
            _new_level: level.count,
        });
    }
}
