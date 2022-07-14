#![allow(clippy::redundant_field_names)]
use bevy::prelude::*;
use bevy::window::PresentMode;

mod components;
mod systems;

use systems::graphics::GraphicsPlugin;
use systems::player::PlayerPlugin;
use systems::camera::spawn_camera;
use systems::debug::DebugPlugin;
use systems::input::InputPlugin;
use systems::ascii::AsciiPlugin;
use systems::tilemap::TileMapPlugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const TILE_SIZE: f32 = 0.2;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: 1600.0,
            height: 900.0,
            title: "Working".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(AsciiPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(TileMapPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(GraphicsPlugin)
        .run();
}
