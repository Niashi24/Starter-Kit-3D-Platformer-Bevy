use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use common::loading::ModelAssets;

pub fn spawn_grass_platform(world: &mut World) {
    let grass = world.resource::<ModelAssets>().grass.clone();
    let grass_small = world.resource::<ModelAssets>().grass_small.clone();
    world.spawn(SceneBundle {
        scene: world.resource::<ModelAssets>().platform_grass_large_round.clone(),
        transform: Transform::from_xyz(-7.0, 1.0, -2.0),
        ..default()
    })
        .insert(Name::new("Grass Platform"))
        .with_children(|c| {
            c.spawn(SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.25, 0.0)))
                .insert(Collider::cylinder(0.25, 2.5));

            c.spawn(SceneBundle {
                scene: grass_small,
                transform: Transform {
                    translation: Vec3::new(-1.263, 0.49, 1.547),
                    rotation: Quat::IDENTITY,
                    scale: Vec3::ONE,
                },
                ..default()
            });

            c.spawn(SceneBundle {
                scene: grass.clone(),
                transform: Transform {
                    translation: Vec3::new(1.389, 0.444, 1.451),
                    rotation: Quat::IDENTITY,
                    scale: Vec3::ONE,
                },
                ..default()
            });

            c.spawn(SceneBundle {
                scene: grass,
                transform: Transform {
                    translation: Vec3::new(0.908, 0.444, -1.671),
                    rotation: Quat::from_euler(
                        EulerRot::XYZ,
                        0.0, 113.8f32.to_radians(), 0.0,
                    ),
                    scale: Vec3::ONE,
                },
                ..default()
            });
        });
}