use std::ops::{Range, RangeInclusive};
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use serde::{Deserialize, Serialize};
use crate::GameState;
use crate::player::input::PlayerAction;
use crate::player::player::{Player, PlayerSystemSet};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_camera)
            .add_systems(Update, (
                track_player.after(PlayerSystemSet),
                gizmos_point,
                (rotate_camera, turn_towards_target).chain(),
                (zoom_camera, zoom_towards_target).chain(),
            ).run_if(in_state(GameState::Playing)))
            
            .register_type::<ViewFollowTarget>()
            
            .register_type::<ViewCamera>()
            .register_type::<ViewZoomStats>()
            .register_type::<TargetZoom>()
            
            .register_type::<ViewRotateStats>()
            .register_type::<TargetRotation>();
    }
}


#[derive(Component, Clone, Reflect, Serialize, Deserialize, Debug)]
pub struct ViewFollowTarget(Entity);

#[derive(Component, Clone, Reflect, Serialize, Deserialize, Debug)]
pub struct ViewZoomStats {
    pub zoom_speed: f32,
    pub zoom_range: RangeInclusive<f32>,
}

impl Default for ViewZoomStats {
    fn default() -> Self {
        Self {
            zoom_speed: 10.0,
            zoom_range: 4.0..=16.0,
        }
    }
}

#[derive(Component, Clone, Reflect, Serialize, Deserialize, Debug)]
struct TargetZoom(f32);

#[derive(Component, Clone, Reflect, Serialize, Deserialize, Debug)]
struct ViewCamera(Entity);

#[derive(Component, Clone, Reflect, Serialize, Deserialize, Debug)]
pub struct ViewRotateStats {
    pub rotation_speed: f32,
    pub pitch_range: RangeInclusive<f32>,
}

impl Default for ViewRotateStats {
    fn default() -> Self {
        Self {
            rotation_speed: 120.0f32.to_radians(),
            pitch_range: (-80f32).to_radians()..=(-10f32).to_radians(),
        }
    }
}

#[derive(Component, Clone, Reflect, Serialize, Deserialize, Debug, Default)]
struct TargetRotation(Quat);


impl Default for TargetZoom {
    fn default() -> Self {
        Self(10.0)
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 10.0),
        ..default()
    }).id();
    
    let pivot = commands.spawn(SpatialBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..default()
    })
        .insert((
            ViewRotateStats::default(),
            TargetRotation::default(),
            ViewZoomStats::default(),
            TargetZoom::default(),
            ViewCamera(camera),
        ))
        .add_child(camera);
}

fn track_player() {}

fn gizmos_point(
    mut gizmos: Gizmos,
    query: Query<&GlobalTransform, With<TargetRotation>>,
) {
    for transform in &query {
        gizmos.cuboid(*transform, Color::RED);
    }
}

fn rotate_camera(
    mut query: Query<(&mut TargetRotation, &ViewRotateStats)>,
    player: Query<&ActionState<PlayerAction>>,
    time: Res<Time>,
) {
    let Ok(player) = player.get_single() else {
        error!("player missing");
        return;
    };
    let Some(input) = player.pressed(&PlayerAction::Look)
        .then(|| player.clamped_axis_pair(&PlayerAction::Look).unwrap().xy())
        .and_then(|x| x.try_normalize()) else {
        return;
    };
    
    for (mut rot, stats) in query.iter_mut() {
        let input = input * (time.delta_seconds() * stats.rotation_speed);        
        let (mut x, mut y, z) = rot.0.to_euler(EulerRot::default());
        x += input.x;
        y += input.y;
        y = y.clamp(*stats.pitch_range.start(), *stats.pitch_range.end());
        rot.0 = Quat::from_euler(EulerRot::default(), x, y, z);
    }
}

fn zoom_camera(
    mut query: Query<(&mut TargetZoom, &ViewZoomStats)>,
    player: Query<&ActionState<PlayerAction>>,
    time: Res<Time>,
) {
    let Ok(player) = player.get_single() else {
        error!("player missing");
        return;
    };
    let Some(input) = player.pressed(&PlayerAction::Zoom)
        .then(|| player.clamped_value(&PlayerAction::Zoom)) else {
        return;
    };
    
    for (mut zoom, stats) in &mut query {
        zoom.0 = (zoom.0 + input * stats.zoom_speed * time.delta_seconds())
            .clamp(*stats.zoom_range.start(), *stats.zoom_range.end());
    }
}

fn turn_towards_target(
    mut query: Query<(&mut Transform, &TargetRotation)>,
    time: Res<Time>,
) {
    for (mut transform, &TargetRotation(target_rotation)) in query.iter_mut() {
        transform.rotation = slerp_time(
            transform.rotation,
            target_rotation,
            16.0,  // decay constant, hope this is fine!!
            time.delta_seconds());
    }
}

fn zoom_towards_target(
    mut camera: Query<&mut Transform>,
    view: Query<(&TargetZoom, &ViewCamera)>,
    time: Res<Time>,
) {
    for (zoom, camera_id) in &view {
        let Ok(mut transform) = camera.get_mut(camera_id.0) else {
            return;
        };

        transform.translation.z = lerp_time(
                transform.translation.z,
                zoom.0,
                16.0,
                time.delta_seconds());
    }
}

// See https://youtu.be/LSNQuFEDOyQ for why we can't just do a regular lerp
fn lerp_time(a: f32, b: f32, decay: f32, delta: f32) -> f32 {
    a.lerp(b, 1.0 - (-decay * delta).exp())
}

fn lerp_time_vec3(a: Vec3, b: Vec3, decay: f32, delta: f32) -> Vec3 {
    a.lerp(b, 1.0 - (-decay * delta).exp())
}

fn slerp_time(a: Quat, b: Quat, decay: f32, delta: f32) -> Quat {
    a.slerp(b, 1.0 - (-decay * delta).exp())
}
