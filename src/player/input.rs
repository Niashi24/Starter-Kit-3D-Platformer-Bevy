use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use serde::{Deserialize, Serialize};

struct PlayerInputPlugin;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    Move,
    Look,
    Jump,
}

impl PlayerAction {
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();
        // gamepad
        input_map.insert(PlayerAction::Move, DualAxis::left_stick());
        input_map.insert(PlayerAction::Look, DualAxis::right_stick());
        input_map.insert(PlayerAction::Jump, GamepadButtonType::South);

        // kbm
        input_map.insert(PlayerAction::Move, VirtualDPad::wasd());
        input_map.insert(PlayerAction::Look, VirtualDPad::arrow_keys());
        input_map.insert(PlayerAction::Jump, KeyCode::Space);

        input_map
    }
}

pub fn player_input_bundle() -> impl Bundle {
    (
        PlayerAction::default_input_map(),
    )
}


