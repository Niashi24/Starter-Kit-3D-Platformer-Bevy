use bevy::prelude::*;
use bevy::prelude::light_consts::lux;
use bevy_rapier3d::prelude::Collider;
use common::loading::{ModelAssets, PlayerAssets};
use common::player::camera::{TargetRotation, TargetZoom, ViewCamera, ViewFollowTarget, ViewRotateStats, ViewZoomStats};
use common::player::input::player_input_bundle;
use common::player::player::{Player, PlayerStats};


pub fn spawn_main_scene(
    world: &mut World,
) {
    let player = spawn_player(world);
    
    let _view = spawn_camera_view(world, player);
    
    spawn_environment(world);
}

fn spawn_player(world: &mut World) -> Entity {
    let player_model = world.resource::<PlayerAssets>().player.clone_weak();

    world.spawn(SpatialBundle {
        transform: Transform::from_translation(Vec3::new(0., 0.5, 1.)),
        ..default()
    })
        .insert((
            Player,
            PlayerStats {
                movement_speed: 4.0,
                jump_strength: 7.0,
            }
        ))
        .insert(player_input_bundle())
        .with_children(|c| {
            c.spawn(SceneBundle {
                scene: player_model,
                ..default()
            });
        }).id()
}

fn spawn_camera_view(world: &mut World, player: Entity) -> Entity {
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
        .add_child(camera).id()
}

fn spawn_environment(
    world: &mut World,
) {
    world
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    world.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::YXZ,
            (-50.0f32).to_radians(),
            (-115.0f32).to_radians(),
            0.0
        )),
        directional_light: DirectionalLight {
            illuminance: lux::DIRECT_SUNLIGHT,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    world.insert_resource(AmbientLight {
        color: Color::Rgba {
            red: 169.0 / 255.0,
            green: 177.0 / 255.0,
            blue: 197.0 / 255.0,
            alpha: 1.0,
        },
        brightness: 10000.0,
    });

    world.insert_resource(ClearColor(Color::Rgba {
        red: 192.0 / 255.0,
        green: 198.0 / 255.0,
        blue: 211.0 / 255.0,
        alpha: 1.0,
    }));

    spawn_platform(world, Transform::from_rotation(Quat::from_euler(
        EulerRot::YXZ,
        (-6.7f32).to_radians(),
        0.0,
        0.0
    )));
}

fn spawn_platform(
    world: &mut World,
    transform: Transform,
) {
    let model = world.resource::<ModelAssets>().platform.clone_weak();
    
    world.spawn(SceneBundle {
        scene: model,
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