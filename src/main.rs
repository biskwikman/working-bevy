#![allow(clippy::redundant_field_names)]
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod components;
mod systems;


use systems::input::InputPlugin;
use components::global::{CLEAR, GameState};
use systems::game::GamePlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(InputPlugin)
        .add_state::<GameState>()
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(GamePlugin)
        .run();
}

// Create window and map, maybe extract to other files later
fn setup() {
    WindowResolution::new(1600.0, 900.0);
}
