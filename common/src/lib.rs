#![allow(clippy::type_complexity)]

pub mod audio;
pub mod loading;
pub mod menu;
pub mod player;
pub mod math;
pub mod falling_platform;

use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_rapier3d::TnuaRapier3dPlugin;
use crate::falling_platform::FallingPlatformPlugin;
use crate::player::camera::CameraPlugin;
use crate::player::PlayerPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            LoadingPlugin,
            MenuPlugin,
            InternalAudioPlugin,
            PlayerPlugin,
            CameraPlugin,
            FallingPlatformPlugin,
            
            RapierPhysicsPlugin::<NoUserData>::default().in_fixed_schedule(),
            TnuaRapier3dPlugin::new(FixedUpdate),
            TnuaControllerPlugin::new(FixedUpdate),
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin,
                RapierDebugRenderPlugin::default(),
                WorldInspectorPlugin::default()
            ));
        }
    }
}
