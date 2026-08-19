use bevy::prelude::*;

use crate::scenes::battlefield::health::KilledEvent;

pub struct CurrencyPlugin;

impl Plugin for CurrencyPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_drop_coin_despawn);
    }
}

#[derive(Resource, Clone, Copy)]
pub struct Currency {
    pub coin: f32,
}

#[derive(Component, Clone, Copy)]
pub struct DropCoin(pub f32);

fn on_drop_coin_despawn(
    event: On<KilledEvent>,
    query: Query<&DropCoin>,
    mut currency: ResMut<Currency>,
) {
    if let Ok(dropped_cooin) = query.get(event.entity) {
        currency.coin += dropped_cooin.0;
    }
}
