use bevy::ui::prelude::*;
use bevy::{color::palettes::tailwind, prelude::*};

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {}
}

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
            row_gap: px(10),
            ..default()
        },
        BackgroundColor(tailwind::INDIGO_900.into()),
    )
}
