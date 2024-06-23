use bevy::prelude::*;
use bevy::prelude::light_consts::lux;
use bevy_rapier3d::prelude::*;
use crate::blueprints::*;

pub fn spawn_main_scene(
    world: &mut World,
) {
    let player = spawn_player(world);

    spawn_camera_view(world, player);

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













