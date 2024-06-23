use bevy::prelude::*;
use common::loading::ModelAssets;

pub fn spawn_cloud(
    world: &mut World,
    transform: Transform,
) {
    // TODO: fade out cloud when camera is close (shader?)
    world.spawn(SceneBundle {
        scene: world.resource::<ModelAssets>().cloud.clone(),
        transform,
        ..default()
    }).insert(Name::new("Cloud"));
}