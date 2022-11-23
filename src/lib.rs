use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::render::texture::ImageSettings;

pub mod systems;
pub mod components;

use systems::debug::DebugPlugin;
use systems::input::InputPlugin;
use systems::menus::MainMenuPlugin;
use systems::game::GamePlugin;

use components::global::GameState;
use components::global::TILE_SIZE;
use components::global::CLEAR;


pub fn run() {
    let mut app = App::new();

    app
        .add_state(GameState::MainMenu)
        .insert_resource(WindowDescriptor {
            width: 1600.0,
            height: 900.0,
            title: "Working".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin);
    // run();
}
