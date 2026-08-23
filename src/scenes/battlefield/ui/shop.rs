use std::sync::Arc;

use bevy::color::palettes::tailwind;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use bevy::ui_widgets::{Activate, observe};

use crate::scenes::battlefield::BattleFieldSet;
use crate::scenes::battlefield::currency::Coins;
use crate::scenes::battlefield::obstacle::Obstacle;
use crate::scenes::battlefield::tower::Tower;
use crate::scenes::battlefield::ui::next_wave::DisabledDuringWave;
use crate::ui::EnableButtonEvent;

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_shop_button.run_if(
                    resource_exists_and_changed::<Coins>
                        .or_else(resource_exists_and_changed::<Shop>),
                ),
                update_shop.run_if(resource_exists_and_changed::<Shop>),
            )
                .in_set(BattleFieldSet),
        );
    }
}

#[derive(Resource)]
pub struct Shop {
    all_items: Vec<Arc<dyn ShopItem>>,
    current_items: Vec<usize>,
}

impl Shop {
    pub fn new() -> Self {
        Self {
            all_items: vec![Arc::new(Tower), Arc::new(Obstacle)],
            current_items: vec![0, 1],
        }
    }
}

pub trait ShopItem: Send + Sync + 'static {
    fn spawn(&self, commands: &mut RelatedSpawnerCommands<'_, ChildOf>);
    fn buy(&self, commands: &mut Commands);
    fn price(&self) -> Price;
}

#[derive(Component)]
struct ShopItemContainer;

#[derive(Component)]
struct ShopButton;

#[derive(Component)]
pub struct Price(pub f32);

#[derive(Component)]
pub struct HadEnoughCoin(bool);

pub fn shop() -> impl Bundle {
    (
        ShopItemContainer,
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
    )
}

fn update_shop(
    mut commands: Commands,
    shop: Res<Shop>,
    item_container: Single<Entity, With<ShopItemContainer>>,
) {
    commands
        .entity(*item_container)
        .despawn_children()
        .with_children(|container| {
            shop.current_items
                .iter()
                .filter_map(|index| shop.all_items.get(*index))
                .for_each(|item| {
                    let cloned_item = item.clone();
                    container
                        .spawn((
                            Button,
                            Node {
                                width: percent(80),
                                height: px(100),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                row_gap: px(10),
                                border: UiRect::all(px(5)),
                                border_radius: BorderRadius::all(percent(10)),
                                ..default()
                            },
                            DisabledDuringWave,
                            ShopButton,
                            item.price(),
                            observe(move |_: On<Activate>, mut commands: Commands| {
                                cloned_item.buy(&mut commands);
                            }),
                        ))
                        .with_children(|button| {
                            item.spawn(button);
                            button
                                .spawn((Text::new("Price"), TextColor(tailwind::SLATE_200.into())));
                        });
                });
        });
}

fn update_shop_button(
    mut commands: Commands,
    button: Query<(Entity, &Price, Option<&mut HadEnoughCoin>, &Children), With<ShopButton>>,
    mut texts: Query<(&mut Text, &mut TextColor)>,
    coins: Res<Coins>,
) {
    for (entity, price, maybe_had_enough_coin, children) in button {
        let has_enough_coin = coins.0 >= price.0;

        let (mut text, mut color) = texts
            .get_mut(children[1])
            .expect("Should have child with Text.");

        **text = format!("{} coins", price.0 as u32);
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
}
