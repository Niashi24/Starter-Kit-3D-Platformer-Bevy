use bevy::input::ButtonInput;
use bevy::math::{EulerRot, Quat, Vec3};
use bevy::prelude::{Camera, GlobalTransform, KeyCode, Query, Res, Time, Transform, With};
use leafwing_input_manager::action_state::ActionState;
use crate::player::input::PlayerAction;
use crate::player::components::{Player, PlayerStats};

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