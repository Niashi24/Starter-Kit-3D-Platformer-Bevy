use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;
use crate::GameState;
use crate::loading::ModelAssets;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_environment);
    }
}

fn spawn_environment(
    mut commands: Commands,
    models: Res<ModelAssets>,
) {
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));
    
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::YXZ,
            (-50.0f32).to_radians(),
            (-115.0f32).to_radians(),
            0.0
        )),
        ..default()
    });
    
    commands.insert_resource(AmbientLight {
        color: Color::Rgba {
            red: 169.0 / 255.0,
            green: 177.0 / 255.0,
            blue: 197.0 / 255.0,
            alpha: 1.0,
        },
        brightness: 10000.0,
    });
    
    commands.insert_resource(ClearColor(Color::Rgba {
        red: 192.0 / 255.0,
        green: 198.0 / 255.0,
        blue: 211.0 / 255.0,
        alpha: 1.0,
    }));
    
    spawn_platform(&mut commands, models, Transform::from_rotation(Quat::from_euler(
        EulerRot::YXZ,
        (-6.7f32).to_radians(),
        0.0,
        0.0
    )));
}

fn spawn_platform(
    commands: &mut Commands,
    models: Res<ModelAssets>,
    transform: Transform,
) {
    commands.spawn(SceneBundle {
        scene: models.platform.clone_weak(),
        transform,
        ..default()
    }).with_children(|c| {
        c.spawn(SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.25, 0.0)))
            .insert(Collider::cuboid(
                2.0,
                0.5,
                2.0
            ));
    });
}