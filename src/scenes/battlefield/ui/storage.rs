use std::sync::Arc;

use bevy::{color::palettes::tailwind, prelude::*, ui_widgets::observe};

use crate::scenes::battlefield::{
    relics::{Relic, RemoveRelic, ricochet::RicochetRelic},
    selection::Selection,
    ui::movable_item::{DropItemInSlot, MovableItem, MovableItemSlot},
};

pub fn storage() -> impl Bundle {
    let relic = Relic(Arc::new(RicochetRelic));
    (
        Node {
            width: px(400),
            height: px(200),
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
        children![(
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
                        commands.trigger(RemoveRelic {
                            relic: relic.clone(),
                            target: selection.entity.unwrap(),
                        });
                    }
                },
            ),
            BackgroundColor(tailwind::INDIGO_950.into()),
            children![(
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
            )]
        )],
    )
}
