use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use common::loading::ModelAssets;

pub fn spawn_medium_platform(
    world: &mut World,
    transform: Transform,
) {
    world.spawn(SceneBundle {
        scene: world.resource::<ModelAssets>().platform_medium.clone(),
        transform,
        ..default()
    })
        .insert(Name::new("Medium Platform"))
        .with_children(|c| {
            c.spawn(SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.25, 0.0)))
                .insert(Collider::cuboid(
                    1.5,
                    0.25,
                    1.5,
                ));
        });
}