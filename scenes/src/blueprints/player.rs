use bevy::core::Name;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_tnua::control_helpers::TnuaSimpleAirActionsCounter;
use bevy_tnua::prelude::*;
use bevy_tnua_rapier3d::{TnuaRapier3dIOBundle, TnuaRapier3dSensorShape};
use common::loading::PlayerAssets;
use common::player::components::{Player, PlayerStats};
use common::player::input::player_input_bundle;

pub fn spawn_player(world: &mut World) -> Entity {
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
