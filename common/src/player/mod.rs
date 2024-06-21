use bevy::app::{App, FixedUpdate, Plugin, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};
use leafwing_input_manager::plugin::InputManagerPlugin;
use crate::GameState;
use crate::player::components::{Gravity, Player, PlayerState, PlayerStats, Velocity};
use crate::player::input::PlayerAction;
use crate::player::systems::*;

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
            .add_systems(FixedUpdate, (
                (
                    update_applied_velocity,
                    check_grounded,
                    transition_air_state,
                    jump,
                    (
                        move_player,
                        apply_player_air_gravity
                    ),
                    apply_current_velocity,
                    rotate_towards_movement
                ).chain(),
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
            .register_type::<Player>()
            .register_type::<PlayerState>()
            .register_type::<Gravity>()
            .register_type::<Velocity>();
    }
}