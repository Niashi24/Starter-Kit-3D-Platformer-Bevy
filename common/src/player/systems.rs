use bevy::input::ButtonInput;
use bevy::math::{EulerRot, Quat, Vec3};
use bevy::prelude::{Camera, GlobalTransform, info, KeyCode, Query, Res, Time, Transform, With};
use bevy_rapier3d::control::KinematicCharacterController;
use leafwing_input_manager::action_state::ActionState;
use crate::player::camera::nudge_velocity_acceleration;
use crate::player::input::PlayerAction;
use crate::player::components::{Player, PlayerStats, Velocity};

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
    mut player_query: Query<(&mut KinematicCharacterController, &mut Velocity, &PlayerStats, &ActionState<PlayerAction>), With<Player>>,
) {
    let camera_rotation = camera.single().to_scale_rotation_translation().1;
    let yaw = camera_rotation.to_euler(EulerRot::YXZ).0;
    let rotation = Quat::from_axis_angle(Vec3::Y, yaw);

    for (mut character_controller, mut velocity, player_stats, input) in player_query.iter_mut() {
        let Some(mut move_delta) = input.pressed(&PlayerAction::Move)
            .then(|| input.clamped_axis_pair(&PlayerAction::Move).unwrap().xy())
            .and_then(|x| x.try_normalize())
            // -y because -Z is forward
            .map(|x| Vec3::new(x.x, 0.0, -x.y)) else {
            continue;
        };

        let wanted_velocity = rotation * move_delta * (time.delta_seconds() * player_stats.movement_speed);

        let mut translation = velocity.0 * time.delta_seconds();

        let (dX, dV) = nudge_velocity_acceleration(velocity.0, wanted_velocity, 10.9, time.delta_seconds());

        translation += dX;
        character_controller.translation = Some(translation);
        velocity.0 += dV;

        info!("translation: {}", translation);


        // move_delta *= time.delta_seconds() * player_stats.movement_speed;
        //
        // player_transform.translation = Some(rotation * move_delta);
    }
}