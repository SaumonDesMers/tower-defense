use crate::scenes::battlefield::ui::currencies::{CurrencyDisplayPlugin, currency_display};
use crate::scenes::battlefield::ui::inspector::{InspectorPlugin, inspector_window};
use crate::scenes::battlefield::ui::main_menu::{MainMenuButtonPlugin, main_menu_button};
use crate::scenes::battlefield::ui::next_wave::{NextWaveButtonPlugin, next_wave_button};
use crate::scenes::battlefield::ui::shop::{ShopPlugin, shop};
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

mod currencies;
mod inspector;
mod main_menu;
mod next_wave;
mod shop;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InspectorPlugin,
            ShopPlugin,
            NextWaveButtonPlugin,
            MainMenuButtonPlugin,
            CurrencyDisplayPlugin,
        ));
    }
}

pub fn new_ui() -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Start,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                // Top Bar
                Node {
                    width: percent(100),
                    height: px(50),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                children![currency_display(), main_menu_button()]
            ),
            (
                Node {
                    width: percent(100),
                    height: percent(100),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                children![
                    shop(),
                    // (
                    //     // Shop
                    //     Node {
                    //         width: px(400),
                    //         height: percent(100),
                    //         display: Display::Flex,
                    //         flex_direction: FlexDirection::Row,
                    //         ..default()
                    //     },
                    //     children![shop()]
                    // ),
                    (
                        Node {
                            width: percent(100),
                            height: px(100),
                            align_self: AlignSelf::End,
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        BackgroundColor(tailwind::PINK_600.into()),
                    ),
                    (
                        Node {
                            width: px(400),
                            height: percent(100),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        children![
                            (
                                // Inspector
                                Node {
                                    width: percent(100),
                                    height: percent(100),
                                    display: Display::Flex,
                                    ..default()
                                },
                                children![inspector_window()]
                            ),
                            next_wave_button()
                        ]
                    )
                ]
            )
        ],
    )
}
