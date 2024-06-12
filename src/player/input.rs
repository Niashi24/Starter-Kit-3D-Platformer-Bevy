use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use serde::{Deserialize, Serialize};

struct PlayerInputPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerInputSet;

#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize, Reflect, Default)]
pub struct PlayerInput {
    dir: Vec2,
    jump: bool,
}

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_input
            ).in_set(PlayerInputSet),
        ).register_type::<PlayerInput>();
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    Move,
    Look,
    Jump,
}

impl Action {
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();
        // gamepad
        input_map.insert(Action::Move, DualAxis::left_stick());
        input_map.insert(Action::Look, DualAxis::right_stick());
        input_map.insert(Action::Jump, GamepadButtonType::South);

        // kbm
        input_map.insert(Action::Move, VirtualDPad::wasd());
        input_map.insert(Action::Look, VirtualDPad::arrow_keys());
        input_map.insert(Action::Jump, KeyCode::Space);

        input_map
    }
}

pub fn player_input_bundle() -> impl Bundle {
    (
        Action::default_input_map(),
        PlayerInput::default(),
    )
}

fn update_input(
    mut query: Query<(&mut PlayerInput, &ActionState<Action>)>,
    camera: Query<&Camera>,
) {
    let camera = camera.single();
    for (mut input, action) in query.iter_mut() {
        // action.
    }
}


