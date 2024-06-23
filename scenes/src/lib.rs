use bevy::prelude::*;
use common::GameState;
use crate::main_scene::spawn_main_scene;

pub mod main_scene;
mod blueprints;

pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_main_scene);
    }
}