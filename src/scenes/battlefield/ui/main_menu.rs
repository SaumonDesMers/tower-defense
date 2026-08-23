use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use bevy::ui_widgets::{Activate, observe};

use crate::scenes::AppState;
pub struct MainMenuButtonPlugin;

impl Plugin for MainMenuButtonPlugin {
    fn build(&self, _app: &mut App) {}
}

pub fn main_menu_button() -> impl Bundle {
    (
        Button,
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
            // border: UiRect::all(px(5)),
            padding: UiRect {
                left: px(15),
                right: px(15),
                top: px(15),
                bottom: px(15),
            },
            ..default()
        },
        children![(
            Text::new("Main Menu"),
            TextColor(tailwind::SLATE_200.into()),
        )],
        observe(
            |_: On<Activate>, mut scene_next_state: ResMut<NextState<AppState>>| {
                scene_next_state.set(AppState::MainMenu);
            },
        ),
    )
}
