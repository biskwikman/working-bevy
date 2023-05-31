#![allow(clippy::redundant_field_names)]
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod components;
mod systems;


use systems::input::InputPlugin;
use components::global::CLEAR;
use systems::game::GamePlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(CLEAR))
        .add_plugin(GamePlugin)
        .add_startup_system(setup)
        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::Index(0))
        .add_plugin(InputPlugin)
        .run();
}

// Create window and map, maybe extract to other files later
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    WindowResolution::new(1600.0, 900.0);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/my_project.ldtk"),
        ..Default::default()
    });
}
