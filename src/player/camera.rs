use bevy::prelude::*;
use crate::GameState;
use crate::player::player::PlayerSystemSet;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_camera)
            .add_systems(Update, track_player.after(PlayerSystemSet));
    }
}

fn spawn_camera(mut commands: Commands) {
    let pivot = commands.spawn(SpatialBundle::default())
        .with_children(|commands| {
            commands.spawn(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.0, 10.0),
                ..default()
            });
        });
}

fn track_player() {
    
}