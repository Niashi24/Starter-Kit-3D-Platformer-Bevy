use bevy::prelude::*;

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect)]
pub struct FallingPlatform;

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Reflect)]
pub struct FallingPlatformSensor(pub Entity);

#[derive(Component, Copy, Clone, PartialEq, Debug, Default, Reflect)]
pub enum FallingPlatformState {
    #[default]
    Fixed,
    Falling {
        gravity: f32,
    },
}