use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use crate::player::input::{Action, player_input_bundle, PlayerInput, PlayerInputSet};

pub struct PlayerPlugin;

#[derive(Component)]
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
                    PlayerSystemSet.after(PlayerInputSet),
                    PlayerSystemSet.run_if(in_state(GameState::Playing)),
                ),
            );
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: textures.bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        })
        .insert(Player)
        .insert(player_input_bundle());
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut Transform, &PlayerInput), With<Player>>,
) {
    for (mut player_transform, input) in &mut player_query {
        // player_transform.translation += movement;
    }
}
