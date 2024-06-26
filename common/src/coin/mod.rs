use bevy::prelude::*;

pub mod components;
pub mod systems;

pub use systems::*;
pub use components::*;

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(FixedUpdate, (
                collect_coin,
                (
                    increment_coin_count,
                    disable_collected_coin,
                )
            ).chain())
            .add_event::<CoinCollected>()
            .init_resource::<CoinCount>()
            .register_type::<Coin>()
            .register_type::<CoinCount>()
            .register_type::<CoinState>()
            .register_type::<CoinCollected>();
    }
}