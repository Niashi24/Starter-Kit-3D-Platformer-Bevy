use bevy::prelude::*;
use bevy::prelude::light_consts::lux;
use bevy_rapier3d::prelude::*;
use bevy_tnua::control_helpers::TnuaSimpleAirActionsCounter;
use bevy_tnua::controller::TnuaControllerBundle;
use bevy_tnua::prelude::{TnuaBuiltinJump, TnuaBuiltinWalk};
use bevy_tnua_rapier3d::{TnuaRapier3dIOBundle, TnuaRapier3dSensorShape};
use common::loading::{ModelAssets, PlayerAssets};
use common::player::camera::{TargetRotation, TargetZoom, ViewCamera, ViewFollowTarget, ViewRotateStats, ViewZoomStats};
use common::player::input::player_input_bundle;
use common::player::components::{Player, PlayerStats};

pub fn spawn_main_scene(
    world: &mut World,
) {
    let player = spawn_player(world);

    let _view = spawn_camera_view(world, player);

    spawn_environment(world);
}

fn spawn_player(world: &mut World) -> Entity {
    let player_model = world.resource::<PlayerAssets>().player.clone_weak();
    world.resource_mut::<RapierConfiguration>().gravity = Vec3::NEG_Y * 25.0;

    world.spawn(SpatialBundle::from_transform(
        Transform::from_translation(Vec3::new(0., 0.5, 1.))))
        .insert((
            Name::new("Player"),
            Player,
            PlayerStats {
                // one ground jump + one air jump
                // OR two air jumps
                num_jumps: 1,
                movement_speed: 4.0,
                jump_strength: 7.0,
                walk: TnuaBuiltinWalk {
                    float_height: 0.26,
                    cling_distance: 0.35,
                    air_acceleration: 20.0,
                    acceleration: 20.0,
                    turning_angvel: 20.0,
                    ..default()
                },
                jump: TnuaBuiltinJump {
                    height: 0.98,
                    ..default()
                }
            },
            TnuaSimpleAirActionsCounter::default(),
            RigidBody::Dynamic,
            TnuaRapier3dIOBundle::default(),
            TnuaControllerBundle::default(),
            Collider::capsule(Vec3::Y * 0.55, Vec3::Y * 0.75, 0.3),
            TnuaRapier3dSensorShape(Collider::capsule(Vec3::Y * 0.55, Vec3::Y * 0.75, 0.28)),
            LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
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
            EulerRot::XYZ,
            (-50.0f32).to_radians(),
            (-130.0f32).to_radians(),
            0.0,
        )),
        directional_light: DirectionalLight {
            illuminance: lux::FULL_DAYLIGHT,
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


    let platforms: [Transform; 4] = [
        Transform {
            translation: Vec3::ZERO,
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                0.0, (-6.7f32).to_radians(), 0.0,
            ),
            scale: Vec3::ONE,
        },
        Transform {
            translation: Vec3::new(-15.0, 0.0, 4.0),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                0.0, (-6.7f32).to_radians(), 0.0,
            ),
            scale: Vec3::ONE,
        },
        Transform {
            translation: Vec3::new(-3.0, 2.0, -3.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        },
        Transform {
            translation: Vec3::new(-3.0, 3.0, -5.0),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                0.0, (-14.9f32).to_radians(), 0.0,
            ),
            scale: Vec3::ONE,
        },
    ];

    for platform in platforms {
        spawn_platform(world, platform);
    }

    let medium_platforms = [
        Transform {
            translation: Vec3::new(-3.0, 0.0, 0.0),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                0.0, 5.0f32.to_radians(), 0.0,
            ),
            scale: Vec3::ONE,
        },
        Transform {
            translation: Vec3::new(-5.0, 0.0, 4.0),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                0.0, 5.0732f32.to_radians(), 0.0,
            ),
            scale: Vec3::ONE,
        },
        Transform {
            translation: Vec3::new(-14.942, 0.992, 0.128),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                0.0, (-21.6f32).to_radians(), 0.0,
            ),
            scale: Vec3::ONE,
        },
        Transform {
            translation: Vec3::new(0.0, 3.0, -6.0),
            rotation: Default::default(),
            scale: Vec3::ONE,
        },
    ];

    for platform in medium_platforms {
        spawn_medium_platform(world, platform);
    }

    let falling_platforms = [
        Transform {
            translation: Vec3::new(-9.0, 0.419, 4.0),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                0.0, 10f32.to_radians(), 0.0,
            ),
            scale: Vec3::ONE,
        },
        Transform {
            translation: Vec3::new(-12.0, 0.315, 4.0),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                0.0, (-6f32).to_radians(), 0.0,
            ),
            scale: Vec3::ONE,
        },
        Transform {
            translation: Vec3::new(-11.753, 1.83, -2.306),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                0.0, 20f32.to_radians(), 0.0,
            ),
            scale: Vec3::ONE,
        },
    ];

    for platform in falling_platforms {
        spawn_falling_platform(world, platform);
    }

    const COINS: [Vec3; 10] = [
        Vec3::new(-3.0, 0.635, 0.0),
        Vec3::new(-5.0, 0.635, 4.0),
        Vec3::new(-7.044, 1.97, -0.33),
        Vec3::new(-7.044, 1.97, -1.33),
        Vec3::new(-11.773, 2.549, -2.282),
        Vec3::new(-14.811, 1.689, 0.329),
        Vec3::new(-14.811, 2.689, 0.329),
        Vec3::new(-14.965, 0.802, 3.994),
        Vec3::new(0.0, 5.0, -6.0),
        Vec3::new(-7.044, 1.97, -2.33),
    ];

    for coin in COINS.into_iter().map(Transform::from_translation) {
        spawn_coin(world, coin);
    }

    spawn_flag(world);

    spawn_grass_platform(world);

    let clouds: [Transform; 7] = [
        Transform::from_xyz(1.549, 1.107, -2.666),
        Transform {
            translation: Vec3::new(3.335, 1.371, -4.193),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                7.3f32.to_radians(), 17.7f32.to_radians(), 19.8f32.to_radians(),
            ),
            scale: Vec3::splat(1.403),
        },
        Transform {
            translation: Vec3::new(-10.575, 2.038, -7.937),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                15.7f32.to_radians(), 11.1f32.to_radians(), (-12.6f32).to_radians(),
            ),
            scale: Vec3::splat(1.403),
        },
        Transform {
            translation: Vec3::new(-11.182, 2.038, 9.281),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                19f32.to_radians(), 45.2f32.to_radians(), (-44.5f32).to_radians(),
            ),
            scale: Vec3::splat(1.403),
        },
        Transform {
            translation: Vec3::new(-10.916, 2.795, 11.515),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                19f32.to_radians(), 150.5f32.to_radians(), (-44.5f32).to_radians(),
            ),
            scale: Vec3::splat(1.403),
        },
        Transform {
            translation: Vec3::new(-14.304, 2.038, -8.242),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                7.3f32.to_radians(), 39.8f32.to_radians(), 47.5f32.to_radians(),
            ),
            scale: Vec3::splat(2.699),
        },
        Transform {
            translation: Vec3::new(-15.866, 2.038, 7.837),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                7.3f32.to_radians(), 39.8f32.to_radians(), 47.5f32.to_radians(),
            ),
            scale: Vec3::splat(2.699),
        },
    ];

    for cloud in clouds {
        spawn_cloud(world, cloud);
    }
}

fn spawn_platform(
    world: &mut World,
    transform: Transform,
) {
    world.spawn(SceneBundle {
        scene: world.resource::<ModelAssets>().platform.clone(),
        transform,
        ..default()
    })
        .insert(Name::new("Falling Platform"))
        .with_children(|c| {
            c.spawn(SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.25, 0.0)))
                .insert(Collider::cuboid(
                    1.0,
                    0.25,
                    1.0,
                ));
        });
}

fn spawn_medium_platform(
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


fn spawn_falling_platform(
    world: &mut World,
    transform: Transform,
) {
    world.spawn(SceneBundle {
        scene: world.resource::<ModelAssets>().platform_falling.clone(),
        transform,
        ..default()
    })
        .insert(Name::new("Platform"))
        .with_children(|c| {
            c.spawn(SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.25, 0.0)))
                .insert(Collider::cuboid(
                    1.0,
                    0.25,
                    1.0,
                ))
                .insert(RigidBody::Fixed);
        });
}

fn spawn_coin(
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

fn spawn_cloud(
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

fn spawn_flag(world: &mut World) {
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

fn spawn_grass_platform(world: &mut World) {
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
