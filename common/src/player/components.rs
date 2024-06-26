use std::default;

use bevy::prelude::*;
use bevy_tnua::prelude::{TnuaBuiltinJump, TnuaBuiltinWalk};
// use serde::{Deserialize, Serialize};

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect)]
pub struct Player;

#[derive(Component, Clone, Default)]
pub struct PlayerStats {
    pub num_jumps: usize,
    pub movement_speed: f32,
    pub jump_strength: f32,
    pub walk: TnuaBuiltinWalk,
    pub jump: TnuaBuiltinJump,
}

#[derive(Copy, Clone, PartialEq, Debug, Default, Reflect)]
pub enum PlayerAnimationState {
    #[default]
    Idle,
    Walk(f32),
    Jump
}
