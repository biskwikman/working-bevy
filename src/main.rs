#![allow(clippy::redundant_field_names)]
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::window::PresentMode;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

mod components;
mod systems;

use systems::game::graphics::GraphicsPlugin;
use systems::game::player::PlayerPlugin;
use systems::game::camera::CameraPlugin;
use systems::debug::DebugPlugin;
use systems::input::InputPlugin;
use systems::npc::NpcPlugin;
use systems::menus::start_menu::StartMenuPlugin;
use components::global::CLEAR;
use components::global::TILE_SIZE;
use components::global::GameState;
use systems::game::GamePlugin;


fn main() {
    App::new()
        .add_state(GameState::MainMenu)
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(ClearColor(CLEAR))
        // .insert_resource(WindowDescriptor {
        //     width: 1600.0,
        //     height: 900.0,
        //     title: "Working".to_string(),
        //     present_mode: PresentMode::Fifo,
        //     resizable: false,
        //     ..Default::default()
        // })
        .add_plugin(GamePlugin)
        .add_startup_system(setup)
        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::Index(0))
        .add_plugin(DebugPlugin)
        .add_plugin(InputPlugin)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/my_project.ldtk"),
        ..Default::default()
    });
}
