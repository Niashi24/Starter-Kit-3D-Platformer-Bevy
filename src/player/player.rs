use crate::actions::Actions;
use crate::loading::{PlayerAssets, TextureAssets};
use crate::GameState;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use serde::{Deserialize, Serialize};
use crate::player::input::{PlayerAction, player_input_bundle};

pub struct PlayerPlugin;

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect)]
pub struct Player;

#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct PlayerSystemSet;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update, (
                    move_player
                ).in_set(PlayerSystemSet))
            .configure_sets(
                Update,
                (
                    PlayerSystemSet.run_if(in_state(GameState::Playing)),
                ),
            ).register_type::<PlayerStats>()
            .register_type::<Player>();
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, Default, Reflect)]
pub struct PlayerStats {
    movement_speed: f32,
    jump_strength: f32,
}


fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    player_assets: Res<PlayerAssets>
) {
    let model = commands.spawn(SceneBundle {
        scene: player_assets.player.clone_weak(),
        ..default()
    }).id();
    
    commands
        .spawn(SpriteBundle {
            texture: textures.bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        })
        .insert((
            Player,
            PlayerStats {
                movement_speed: 250.0,
                jump_strength: 7.0,
            }
        ))
        .insert(player_input_bundle())
        .insert_children(0, &[model]);
}

fn move_player(
    time: Res<Time>,
    camera: Query<&GlobalTransform, With<Camera>>,
    mut player_query: Query<(&mut Transform, &ActionState<PlayerAction>), With<Player>>,
) {
    // let camera = camera.single();

    for (mut player_transform, input) in player_query.iter_mut() {
        if let Some(mut move_delta) = input.pressed(&PlayerAction::Move)
            .then(|| input.clamped_axis_pair(&PlayerAction::Move).unwrap().xy()) {
            move_delta *= time.delta_seconds();
        }


        // player_transform.translation += movement;
    }
}
