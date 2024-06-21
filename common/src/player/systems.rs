use bevy::input::ButtonInput;
use bevy::math::{EulerRot, Quat, Vec3};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use crate::math::{look_to, nudge};
use crate::player::camera::{nudge_velocity_acceleration, slerp_time};
use crate::player::input::PlayerAction;
use crate::player::components::{Gravity, Player, PlayerState, PlayerStats, Velocity};

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

pub fn check_grounded(
    physics: Res<RapierContext>,
    mut player: Query<(&mut KinematicCharacterControllerOutput, &KinematicCharacterController, &GlobalTransform, Entity)>,
) {
    for (mut output, controller, transform, entity) in player.iter_mut() {
        let ray_origin = transform.translation();
        let ray_dir = -controller.up;
        let max_toi = 0.05;
        let solid = true;
        let filter = QueryFilter::default().exclude_collider(entity);

        output.grounded = physics.cast_ray(ray_origin, ray_dir, max_toi, solid, filter).is_some();
    }
}

pub fn update_applied_velocity(
    mut player_query: Query<(&mut Velocity, &KinematicCharacterControllerOutput)>,
    time: Res<Time>,
) {
    for (mut vel, output) in player_query.iter_mut() {
        info!("{:?}", output);
        let dS = output.effective_translation - output.desired_translation;
        vel.0 += dS / time.delta_seconds();
    }
}

pub fn apply_current_velocity(
    mut player_query: Query<(&mut KinematicCharacterController, &Velocity)>,
    time: Res<Time>,
) {
    for (mut character, velocity) in player_query.iter_mut() {
        *character.translation.get_or_insert(Vec3::ZERO) += velocity.0 * time.delta_seconds();
    }
}

pub fn move_player(
    time: Res<Time>,
    camera: Query<&GlobalTransform, With<Camera>>,
    mut player_query: Query<(&mut Velocity, &PlayerStats, &ActionState<PlayerAction>), With<Player>>,
) {
    let camera_rotation = camera.single().to_scale_rotation_translation().1;
    let yaw = camera_rotation.to_euler(EulerRot::YXZ).0;
    let rotation = Quat::from_axis_angle(Vec3::Y, yaw);

    for (mut velocity, player_stats, input) in player_query.iter_mut() {
        let move_delta = input.pressed(&PlayerAction::Move)
            .then(|| input.clamped_axis_pair(&PlayerAction::Move).unwrap().xy())
            .map(|x| x.normalize_or_zero())
            // -y because -Z is forward
            .map(|x| Vec3::new(x.x, 0.0, -x.y))
            .unwrap_or(Vec3::ZERO);

        let target_velocity = rotation * move_delta * player_stats.movement_speed;
        let target_velocity = Vec2::new(target_velocity.x, target_velocity.z);
        let current_velocity = Vec2::new(velocity.0.x, velocity.0.z);
        let mut new_velocity = nudge(current_velocity, target_velocity, 10.95, time.delta_seconds());
        new_velocity -= current_velocity;
        let new_velocity = Vec3::new(new_velocity.x, 0.0, new_velocity.y);

        // let (d_x, d_v) = nudge_velocity_acceleration(velocity.0, wanted_velocity, 10.95, time.delta_seconds());

        velocity.0 += new_velocity;
    }
}

pub fn apply_player_air_gravity(
    mut query: Query<(&mut Velocity, &mut KinematicCharacterController, &Gravity, &PlayerState)>,
    time: Res<Time>,
) {
    for (mut velocity, mut character, gravity, state) in query.iter_mut() {
        if state == &PlayerState::Grounded {
            continue;
        }
        
        let d_v = -character.up * gravity.0 * time.delta_seconds();
        let d_x = d_v / 2.0 * time.delta_seconds();
        
        // *character.translation.get_or_insert(Vec3::ZERO) += d_x;
        velocity.0 += d_v;
    }
}

pub fn rotate_towards_movement(
    mut player: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in player.iter_mut() {
        let mut flat = -velocity.0;
        flat.y = 0.0;
        if flat.length_squared() <= 1e-4 {
            continue;
        }

        let target = look_to(flat, Vec3::Y);

        transform.rotation = slerp_time(
            transform.rotation,
            target,
            10.95,
            time.delta_seconds(),
        );
    }
}

pub fn transition_air_state(
    mut player: Query<(&mut PlayerState, &mut Velocity, &KinematicCharacterControllerOutput)>,
) {
    for (mut state, mut velocity, character) in player.iter_mut() {
        println!("{} {}", character.desired_translation.y, character.grounded);
        match (&*state, character.grounded) {
            (PlayerState::Grounded, false) => {
                *state = PlayerState::Airborne {
                    jumps: 2,
                };
            }
            (PlayerState::Airborne { .. }, true) => {
                *state = PlayerState::Grounded;
                velocity.0.y = 0.0;
            }
            (_, _) => {}
        }
    }
}

pub fn jump(
    mut player: Query<(&mut PlayerState, &mut Velocity, &PlayerStats, &ActionState<PlayerAction>)>,
) {
    for (mut state, mut vel, stats, input) in player.iter_mut() {
        if !input.just_pressed(&PlayerAction::Jump) {
            continue;
        }
        
        if !state.can_jump() {
            continue;
        }
        
        state.jump();
        vel.0.y = stats.jump_strength;
    }
}
