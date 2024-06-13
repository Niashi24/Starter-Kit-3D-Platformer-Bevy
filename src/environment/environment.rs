use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;
use crate::GameState;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_environment);
    }
}

fn spawn_environment(
    mut commands: Commands,
) {
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));
    
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::YXZ,
            (-50.0f32).to_radians(),
            (115.0f32).to_radians(),
            0.0
        )),
        ..default()
    });
}