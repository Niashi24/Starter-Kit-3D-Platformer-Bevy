use crate::GameState;
use bevy::prelude::*;
use leafwing_input_manager::prelude::{ActionState, InputManagerPlugin};
use serde::{Deserialize, Serialize};
use crate::player::input::PlayerAction;

pub struct PlayerPlugin;

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect)]
pub struct Player;

#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct PlayerSystemSet;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(
                Update, (
                    move_player
                ).in_set(PlayerSystemSet))
            .add_systems(Update, (
                reset_player
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

#[derive(Component, Debug, Clone, Serialize, Deserialize, Default, Reflect)]
pub struct PlayerStats {
    pub movement_speed: f32,
    pub jump_strength: f32,
}

fn reset_player(
    mut query: Query<&mut Transform, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        for mut trans in query.iter_mut() {
            trans.translation = Vec3::ZERO;
        }
    }
}

fn move_player(
    time: Res<Time>,
    camera: Query<&GlobalTransform, With<Camera>>,
    mut player_query: Query<(&mut Transform, &PlayerStats, &ActionState<PlayerAction>), With<Player>>,
) {
    let camera_rotation = camera.single().to_scale_rotation_translation().1;
    let yaw = camera_rotation.to_euler(EulerRot::YXZ).0;
    let rotation = Quat::from_axis_angle(Vec3::Y, yaw);

    for (mut player_transform, player_stats, input) in player_query.iter_mut() {
        let Some(mut move_delta) = input.pressed(&PlayerAction::Move)
            .then(|| input.clamped_axis_pair(&PlayerAction::Move).unwrap().xy())
            .and_then(|x| x.try_normalize())
            // -y because -Z is forward
            .map(|x| Vec3::new(x.x, 0.0, -x.y)) else {
            continue;
        };
        
        move_delta *= time.delta_seconds() * player_stats.movement_speed;
        
        player_transform.translation += rotation * move_delta;
    }
}
