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


