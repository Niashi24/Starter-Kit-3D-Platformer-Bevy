use bevy::prelude::*;
use common::player::camera::*;

pub fn spawn_camera_view(world: &mut World, player: Entity) {
    let camera = world.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 10.0),
        ..default()
    }).id();

    world.spawn(SpatialBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..default()
    })
        .insert((
            ViewFollowTarget(player),
            ViewRotateStats::default(),
            TargetRotation::default(),
            ViewZoomStats::default(),
            TargetZoom::default(),
            ViewCamera(camera),
        ))
        .add_child(camera);
}