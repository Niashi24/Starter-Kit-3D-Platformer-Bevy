use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use common::falling_platform::components::{FallingPlatform, FallingPlatformSensor, FallingPlatformState};
use common::loading::ModelAssets;

pub fn spawn_falling_platform(
    world: &mut World,
    mut transform: Transform,
) {
    transform.translation += Vec3::Y * 0.25;
    let sensor = world.spawn(SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.45, 0.0)))
        .insert(Collider::cuboid(1.0, 0.05, 1.0))
        .insert(Sensor).id();
    
    let model = world.spawn(SceneBundle {
        scene: world.resource::<ModelAssets>().platform_falling.clone(),
        transform: Transform::from_xyz(0.0, -0.25, 0.0),
        ..default()
    }).id();
    
    world.spawn(SpatialBundle::from_transform(transform))
        .insert(Collider::cuboid(
            1.0,
            0.25,
            1.0,
        ))
        .insert(Velocity::default())
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Name::new("Platform"))
        .insert(FallingPlatform)
        .insert(FallingPlatformSensor(sensor))
        .insert(FallingPlatformState::default())
        .add_child(sensor)
        .add_child(model);
}