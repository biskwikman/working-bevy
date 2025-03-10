#![allow(clippy::redundant_field_names)]
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod components;
mod systems;

use components::global::{GameState, CLEAR};
use systems::game::GamePlugin;
use systems::input::InputPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(InputPlugin)
        .init_state::<GameState>()
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(GamePlugin)
        .run();
}

// Create window and map, maybe extract to other files later
fn setup() {
    WindowResolution::new(1600.0, 900.0);
}
