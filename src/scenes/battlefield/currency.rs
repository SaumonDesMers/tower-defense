use bevy::prelude::*;

use crate::scenes::battlefield::health::KilledEvent;

pub struct CurrencyPlugin;

impl Plugin for CurrencyPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_drop_coin_despawn);
    }
}

#[derive(Resource, Clone, Copy)]
pub struct Coins(pub f32);

#[derive(Component, Clone, Copy)]
pub struct DropCoins(pub f32);

fn on_drop_coin_despawn(
    event: On<KilledEvent>,
    query: Query<&DropCoins>,
    mut coins: ResMut<Coins>,
) {
    if let Ok(dropped_coins) = query.get(event.entity) {
        coins.0 += dropped_coins.0;
    }
}
