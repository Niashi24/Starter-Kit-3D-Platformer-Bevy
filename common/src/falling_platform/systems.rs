use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::falling_platform::components::{FallingPlatform, FallingPlatformSensor, FallingPlatformState};
use crate::player::components::Player;

pub fn fall_if_entered(
    rapier_context: Res<RapierContext>,
    player: Query<Entity, With<Player>>,
    mut platform: Query<(&mut FallingPlatformState, &FallingPlatformSensor), With<FallingPlatform>>
) {
    for player in player.iter() {
        for (mut state, sensor) in platform.iter_mut() {
            if *state != FallingPlatformState::Fixed {
                continue;
            }
            
            if rapier_context.intersection_pair(player, sensor.0) == Some(true) {
                *state = FallingPlatformState::Falling {
                    gravity: 0.0,
                };
            }
        }
    }
}

pub fn add_gravity(
    mut platform: Query<(&mut Velocity, &mut FallingPlatformState)>,
    time: Res<Time>,
) {
    for (mut vel, mut state) in platform.iter_mut() {
        if let FallingPlatformState::Falling { gravity } = state.as_mut() {
            const GRAV: f32 = 0.25;
            *gravity += GRAV * time.delta_seconds();
            vel.linvel.y -= *gravity;
            vel.linvel.y -= 0.5 * GRAV * time.delta_seconds() * time.delta_seconds();
        }
    }
}

pub fn despawn_if_below(
    mut commands: Commands,
    platform: Query<(Entity, &GlobalTransform), With<FallingPlatform>>
) {
    for (platform, transform) in platform.iter() {
        if transform.translation().y < -10.0 {
            commands.entity(platform).despawn_recursive();
        }
    }
}