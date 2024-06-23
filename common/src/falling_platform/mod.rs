use bevy::prelude::*;
use crate::falling_platform::components::*;
use crate::falling_platform::systems::*;

pub mod components;
pub mod systems;

pub struct FallingPlatformPlugin;

impl Plugin for FallingPlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (
            fall_if_entered,
            add_gravity,
            despawn_if_below
        ))
            .register_type::<FallingPlatformState>()
            .register_type::<FallingPlatformSensor>()
            .register_type::<FallingPlatform>();
    }
}