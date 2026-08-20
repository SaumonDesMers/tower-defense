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
pub struct UpgradeEvent {
    pub entity: Entity,
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
            children![
                (
                    upgrade_button(),
                    children![(
                        Text::new("Upgrade 1"),
                        TextColor(tailwind::SLATE_200.into()),
                    )],
                    observe(|_: On<Activate>| {},),
                ),
                (
                    upgrade_button(),
                    children![(
                        Text::new("Upgrade 2"),
                        TextColor(tailwind::SLATE_200.into()),
                    )],
                    observe(|_: On<Activate>| {},),
                ),
                (
                    upgrade_button(),
                    children![(
                        Text::new("Upgrade 3"),
                        TextColor(tailwind::SLATE_200.into()),
                    )],
                    observe(|_: On<Activate>| {},),
                )
            ]
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
    _: On<UpgradeEvent>,
    mut upgrade_menu_visibility: Query<&mut Visibility, With<UpgradeMenuTag>>,
) {
    let mut visibility = upgrade_menu_visibility
        .single_mut()
        .expect("Upgrade menu should exist");
    *visibility = Visibility::Inherited;
}
