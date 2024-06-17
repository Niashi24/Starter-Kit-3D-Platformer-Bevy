use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::GameState;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<AudioAssets>()
                .load_collection::<TextureAssets>()
                .load_collection::<PlayerAssets>()
                .load_collection::<ModelAssets>(),
        );
    }
}

// pub fn load_scene(mut commands: Commands, asset_server: AssetServer) {
//     commands.spawn(DynamicSceneBundle {
//         scene: asset_server.load("scenes/test-scene.ron"),
//         ..default()
//     });
// }

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(path = "models/character.glb#Scene0")]
    pub player: Handle<Scene>,
    #[asset(path = "models/character.glb#Animation0")]
    pub r#static: Handle<AnimationClip>,
    #[asset(path = "models/character.glb#Animation0")]
    pub idle: Handle<AnimationClip>,
    #[asset(path = "models/character.glb#Animation0")]
    pub walk: Handle<AnimationClip>,
    #[asset(path = "models/character.glb#Animation0")]
    pub jump: Handle<AnimationClip>,
}

#[derive(AssetCollection, Resource)]
pub struct ModelAssets {
    #[asset(path = "models/cloud.glb#Scene0")]
    pub cloud: Handle<Scene>,
    #[asset(path = "models/coin.glb#Scene0")]
    pub coin: Handle<Scene>,
    #[asset(path = "models/dust.glb#Scene0")]
    pub dust: Handle<Scene>,
    #[asset(path = "models/flag.glb#Scene0")]
    pub flag: Handle<Scene>,
    #[asset(path = "models/grass.glb#Scene0")]
    pub grass: Handle<Scene>,
    #[asset(path = "models/grass-small.glb#Scene0")]
    pub grass_small: Handle<Scene>,
    #[asset(path = "models/platform.glb#Scene0")]
    pub platform: Handle<Scene>,
    #[asset(path = "models/platform-falling.glb#Scene0")]
    pub platform_falling: Handle<Scene>,
    #[asset(path = "models/platform-grass-large-round.glb#Scene0")]
    pub platform_grass_large_round: Handle<Scene>,
    #[asset(path = "models/platform-large.glb#Scene0")]
    pub platform_large: Handle<Scene>,
    #[asset(path = "models/platform-medium.glb#Scene0")]
    pub platform_medium: Handle<Scene>,
}
