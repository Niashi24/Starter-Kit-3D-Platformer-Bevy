use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect)]
pub struct Player;

#[derive(Component, Debug, Clone, Serialize, Deserialize, Default, Reflect)]
pub struct PlayerStats {
    pub movement_speed: f32,
    pub jump_strength: f32,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, Default, Reflect)]
pub struct Velocity(pub Vec3);

#[derive(Component, Debug, Clone, Serialize, Deserialize, Default, Reflect)]
pub struct Gravity(pub f32);

#[derive(Component, Debug, Clone, Serialize, Deserialize, Default, Reflect, Eq, PartialEq)]
pub enum PlayerState {
    #[default]
    Grounded,
    Airborne {
        jumps: usize
    }
}

impl PlayerState {
    pub fn airborne() -> Self {
        Self::Airborne {
            jumps: 2,
        }
    }
    
    pub fn can_jump(&self) -> bool {
        match self {
            PlayerState::Grounded => true,
            PlayerState::Airborne { jumps: 0 } => false,
            _ => true,
        }
    }
    
    pub fn jump(&mut self) {
        match self {
            PlayerState::Grounded => *self = PlayerState::Airborne { jumps: 1 },
            PlayerState::Airborne { jumps } => { *jumps = jumps.saturating_sub(1) }
        }
    }
}

