use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use common::loading::ModelAssets;

pub fn spawn_coin(
    world: &mut World,
    transform: Transform,
) {
    world.spawn(SceneBundle {
        scene: world.resource::<ModelAssets>().coin.clone(),
        transform: transform.with_translation(transform.translation + Vec3::Y * 0.25),
        ..default()
    })
        .insert(Name::new("Coin"))
        .with_children(|c| {
            c.spawn(SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.25, 0.0)))
                .insert(Collider::ball(0.5f32))
                .insert(Sensor);
        });
}