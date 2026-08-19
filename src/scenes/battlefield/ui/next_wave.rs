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

pub struct NextWaveButtonPlugin;

impl Plugin for NextWaveButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_map_validity_display.run_if(resource_exists_and_changed::<MapValidity>),
                update_next_wave_button.run_if(resource_exists_and_changed::<MapValidity>),
            )
                .in_set(BattleFieldSet),
        );
    }
}

#[derive(Component)]
struct NextWaveButton;

#[derive(Component)]
struct MapValidityDisplay;

pub fn next_wave_button() -> impl Bundle {
    (
        Node {
            width: px(200),
            height: px(200),
            align_self: AlignSelf::End,
            border_radius: BorderRadius::all(px(10)),
            margin: UiRect {
                right: px(10),
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
                MapValidityDisplay,
                Text::new("placeholder"),
                TextColor(tailwind::SLATE_200.into()),
            ),
            (
                NextWaveButton,
                button("Next Wave"),
                observe(|_: On<Activate>, mut commands: Commands| {
                    commands.trigger(UpdatePathfindingMapEvent);
                    commands.trigger(LaunchWaveEvent);
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

fn update_map_validity_display(
    map_validity: Res<MapValidity>,
    mut display: Query<&mut Text, With<MapValidityDisplay>>,
) {
    if let Ok(mut text) = display.single_mut() {
        **text = match &map_validity.error {
            Some(msg) => format!("{}", msg),
            None => format!("Map Valid"),
        };
    }
}

fn update_next_wave_button(
    mut commands: Commands,
    mut button: Query<Entity, With<NextWaveButton>>,
    map_validity: Res<MapValidity>,
    mut map_was_valid: Local<Option<bool>>,
) {
    if let Ok(entity) = button.single_mut() {
        let map_is_valid = map_validity.error.is_none();
        if *&map_was_valid.is_none()
            || (*map_was_valid).expect("Should not execute if None because of previous condition")
                != map_is_valid
        {
            *map_was_valid = Some(map_is_valid);
            commands.trigger(EnableButtonEvent {
                entity,
                enable: map_is_valid,
            });
        }
    }
}
