use std::marker::PhantomData;
use std::sync::Arc;

use bevy::ui::prelude::*;
use bevy::ui_widgets::{Activate, observe};
use bevy::{color::palettes::tailwind, prelude::*};

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_upgrade);
    }
}

#[derive(EntityEvent)]
pub struct OpenUpgradeMenuEvent {
    pub entity: Entity,
}

#[derive(Component)]
pub struct PossibleUpgrades {
    upgrades: Vec<Arc<dyn UpgradeEvent>>,
}

impl PossibleUpgrades {
    pub fn new(upgrades: Vec<impl UpgradeEvent>) -> Self {
        Self {
            upgrades: upgrades
                .into_iter()
                .map(|upgrade| Arc::new(upgrade) as Arc<dyn UpgradeEvent>)
                .collect(),
        }
    }
}

pub trait UpgradeEvent: Send + Sync + 'static {
    fn trigger(&self, commands: &mut Commands, entity: Entity);

    fn name(&self) -> String;
}

#[derive(Component)]
struct UpgradeMenuTag;

pub fn upgrade_menu() -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::from(tailwind::NEUTRAL_950).with_alpha(0.8)),
        UpgradeMenuTag,
        Visibility::Hidden,
        GlobalZIndex(1),
        children![(
            Node {
                width: px(300),
                height: px(500),
                border_radius: BorderRadius::all(px(10)),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                justify_content: JustifyContent::Center,
                row_gap: px(20),
                padding: UiRect {
                    left: px(20),
                    right: px(20),
                    top: px(20),
                    bottom: px(20),
                },
                ..default()
            },
            BackgroundColor(Color::from(tailwind::INDIGO_900)),
        )],
    )
}

fn upgrade_button() -> impl Bundle {
    (
        Button,
        Node {
            height: percent(100),
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
    )
}

fn on_upgrade(
    event: On<OpenUpgradeMenuEvent>,
    mut commands: Commands,
    mut upgrade_menu_visibility: Query<(&mut Visibility, &Children), With<UpgradeMenuTag>>,
    possible_upgrades_q: Query<&PossibleUpgrades>,
) {
    let Ok(possible_upgrades) = possible_upgrades_q.get(event.entity) else {
        warn!("You tried to upgrade an entity that doesn't any possible upgrade");
        return;
    };

    let (mut visibility, children) = upgrade_menu_visibility
        .single_mut()
        .expect("Upgrade menu should exist");

    let upgrades = possible_upgrades
        .upgrades
        .iter()
        .map(|upgrade| upgrade.clone());
    commands
        .entity(children[0])
        .despawn_children()
        .with_children(|parent| {
            for upgrade in upgrades {
                let entity = event.entity;
                parent.spawn((
                    upgrade_button(),
                    children![(
                        Text::new(upgrade.name()),
                        TextColor(tailwind::SLATE_200.into()),
                    )],
                    observe(
                        move |_: On<Activate>,
                              mut commands: Commands,
                              mut upgrade_menu_visibility: Query<
                            &mut Visibility,
                            With<UpgradeMenuTag>,
                        >| {
                            upgrade.trigger(&mut commands, entity);
                            let mut visibility = upgrade_menu_visibility
                                .single_mut()
                                .expect("Upgrade menu should exist");
                            *visibility = Visibility::Hidden;
                        },
                    ),
                ));
            }
        });

    *visibility = Visibility::Inherited;
}
