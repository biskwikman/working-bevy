#![allow(clippy::redundant_field_names)]
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::window::PresentMode;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

mod components;
mod systems;

use systems::graphics::GraphicsPlugin;
use systems::player::PlayerPlugin;
use systems::camera::CameraPlugin;
use systems::debug::DebugPlugin;
use systems::input::InputPlugin;
use systems::npc::NpcPlugin;
// use systems::collision::CollidePlugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const TILE_SIZE: f32 = 0.2;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    StartMenu,
    Overworld,
    Combat,
}

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: 1600.0,
            height: 900.0,
            title: "Working".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(CameraPlugin)
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::Index(0))
        .add_plugin(PlayerPlugin)
        .add_plugin(NpcPlugin)
        .add_plugin(DebugPlugin)
        // .add_plugin(TileMapPlugin)
        .add_plugin(InputPlugin)
        // .add_plugin(CollidePlugin)
        .add_plugin(GraphicsPlugin)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/my_project.ldtk"),
        ..Default::default()
    });
}
