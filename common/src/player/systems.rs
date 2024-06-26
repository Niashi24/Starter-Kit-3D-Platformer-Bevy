﻿use bevy::input::ButtonInput;
use bevy::math::{EulerRot, Quat, Vec3};
use bevy::prelude::*;
use bevy_tnua::control_helpers::TnuaSimpleAirActionsCounter;
use bevy_tnua::prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController};
use bevy_tnua::{TnuaAction, TnuaAnimatingState, TnuaAnimatingStateDirective};
use leafwing_input_manager::action_state::ActionState;
use crate::loading::PlayerAssets;
use crate::player::input::PlayerAction;
use crate::player::components::{Player, PlayerStats};

use super::components::PlayerAnimationState;

pub fn reset_player(
    mut query: Query<&mut Transform, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        for mut trans in query.iter_mut() {
            trans.translation = Vec3::ZERO;
        }
    }
}

pub fn move_player(
    camera: Query<&GlobalTransform, With<Camera>>,
    mut player_query: Query<(
        &mut TnuaController,
        &mut TnuaSimpleAirActionsCounter,
        &PlayerStats,
        &ActionState<PlayerAction>
    ), With<Player>>,
) {
    let camera_rotation = camera.single().to_scale_rotation_translation().1;
    let yaw = camera_rotation.to_euler(EulerRot::YXZ).0;
    let rotation = Quat::from_axis_angle(Vec3::Y, yaw);

    for (mut controller, mut air_counter, player_stats, input) in player_query.iter_mut() {
        air_counter.update(controller.as_ref());
        
        let move_dir = rotation *
            input.pressed(&PlayerAction::Move)
                .then(|| input.clamped_axis_pair(&PlayerAction::Move).unwrap().xy())
                .map(|x| x.clamp_length_max(1.0))
                // -y because -Z is forward
                .map(|x| Vec3::new(x.x, 0.0, -x.y))
                .unwrap_or(Vec3::ZERO);

        controller.basis(TnuaBuiltinWalk {
            desired_velocity: move_dir * player_stats.movement_speed,
            desired_forward: -move_dir.normalize_or_zero(),
            ..player_stats.walk.clone()
        });
        
        // Note: action is passed continuously while jump is held
        if input.pressed(&PlayerAction::Jump) {
            controller.action(TnuaBuiltinJump {
                allow_in_air: air_counter.air_count_for(TnuaBuiltinJump::NAME)
                    <= player_stats.num_jumps,
                ..player_stats.jump.clone()
            });
        }
    }
}

pub fn animate_player(
    mut query: Query<(&mut TnuaAnimatingState<PlayerAnimationState>, &TnuaController, &mut AnimationPlayer, &PlayerStats)>,
    player_animations: Res<PlayerAssets>,
) {
    for (mut state, controller, mut player, stats) in query.iter_mut() {
        info!("here!");

        match state.update_by_discriminant({
            let Some((_, basis_state)) = controller.concrete_basis::<TnuaBuiltinWalk>() else {
                continue;
            };

            let speed = basis_state.running_velocity.length() / stats.movement_speed;


            if basis_state.standing_on_entity().is_none() {
                PlayerAnimationState::Jump
            } else if speed < 0.05 {
                PlayerAnimationState::Idle
            } else {
                PlayerAnimationState::Walk(speed)
            }
            
        }) {
            TnuaAnimatingStateDirective::Maintain { state } => {
                if let PlayerAnimationState::Walk(speed) = state {
                    player.set_speed(*speed);
                }
            },
            TnuaAnimatingStateDirective::Alter { old_state: _, state } => {
                match state {
                    PlayerAnimationState::Idle => player.play(player_animations.idle.clone()).set_speed(1.0),
                    PlayerAnimationState::Walk(s) => player.play(player_animations.walk.clone()).set_speed(*s),
                    PlayerAnimationState::Jump => player.play(player_animations.jump.clone()).set_speed(1.0),
                };
            },
        }
    }
}