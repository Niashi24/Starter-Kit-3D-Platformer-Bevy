use bevy::prelude::*;
use common::loading::ModelAssets;

pub fn spawn_flag(world: &mut World) {
    world.spawn(SceneBundle {
        scene: world.resource::<ModelAssets>().flag.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 3.481, -6.0),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                0.0, (-45f32).to_radians(), 0.0,
            ),
            scale: Vec3::ONE,
        },
        ..default()
    }).insert(Name::new("Flag"));
}