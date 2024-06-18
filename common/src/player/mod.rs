use bevy::app::{App, FixedUpdate, Plugin, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};
use leafwing_input_manager::plugin::InputManagerPlugin;
use crate::GameState;
use crate::player::components::{Player, PlayerStats};
use crate::player::input::PlayerAction;
use crate::player::systems::{move_player, reset_player};

pub mod input;
pub mod camera;
pub mod systems;
pub mod components;
pub struct PlayerPlugin;

#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct PlayerSystemSet;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Update, (
                move_player,
                reset_player,
            ).in_set(PlayerSystemSet))
            .configure_sets(
                Update,
                (
                    PlayerSystemSet.run_if(in_state(GameState::Playing)),
                ))
            .configure_sets(
                FixedUpdate,
                (
                    PlayerSystemSet.run_if(in_state(GameState::Playing)),
                ))
            .register_type::<PlayerStats>()
            .register_type::<Player>();
    }
}