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

pub struct CurrencyDisplayPlugin;

impl Plugin for CurrencyDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_currency_display.run_if(resource_exists_and_changed::<Currency>),)
                .in_set(BattleFieldSet),
        );
    }
}

#[derive(Component)]
struct CurrencyDisplay;

pub fn currency_display() -> impl Bundle {
    (
        Node {
            width: px(300),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            border_radius: BorderRadius::bottom_right(px(10)),
            ..default()
        },
        BackgroundColor(tailwind::INDIGO_900.into()),
        children![(
            CurrencyDisplay,
            Text::new("placeholder"),
            TextColor(tailwind::SLATE_200.into()),
        ),],
    )
}

fn update_currency_display(
    currency: Res<Currency>,
    mut display: Query<&mut Text, With<CurrencyDisplay>>,
) {
    if let Ok(mut text) = display.single_mut() {
        **text = format!(
            "Coin: {}    Xp: {}",
            currency.coin as u32, currency.xp as u32
        );
    }
}
