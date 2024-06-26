use bevy::prelude::*;

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect)]
pub struct Coin;

#[derive(Resource, Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect)]
pub struct CoinCount(pub usize);

impl CoinCount {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
    
    pub fn reset(&mut self) {
        self.0 = 0;
    }
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect)]
pub enum CoinState {
    #[default]
    Available,
    Collected,
}

#[derive(Event, Copy, Clone, Eq, PartialEq, Hash, Debug, Reflect)]
pub struct CoinCollected(pub Entity);
