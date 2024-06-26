use bevy::prelude::*;
use bevy_rapier3d::plugin::RapierContext;
use super::components::*;
use crate::player::components::Player;

pub fn increment_coin_count(
    mut coin_collected: EventReader<CoinCollected>,
    mut coin_count: ResMut<CoinCount>,
) {
    for _ in coin_collected.read() {
        coin_count.increment();
        info!("{} coins collected.", coin_count.0);
    }
}

pub fn collect_coin(
    mut coin_collected: EventWriter<CoinCollected>,
    coins: Query<(Entity, &CoinState), With<Coin>>,
    player: Query<Entity, With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    for player in player.iter() {
        for (coin, state) in coins.iter() {
            if *state == CoinState::Collected { continue; }

            if rapier_context.intersection_pair(player, coin) == Some(true) {
                coin_collected.send(CoinCollected(coin));
            }
        }
    }
}

pub fn disable_collected_coin(
    mut coin_collected: EventReader<CoinCollected>,
    mut coins: Query<(&mut CoinState, &mut Visibility)>,
) {
    for &CoinCollected(coin) in coin_collected.read() {
        let (mut state, mut visibility) = coins.get_mut(coin).unwrap();
        
        *state = CoinState::Collected;
        *visibility = Visibility::Hidden;
    }
}


