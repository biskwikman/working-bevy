use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_ecs_ldtk::prelude::*;
use std::{fs::File, io::{BufReader, BufRead}, collections::VecDeque};
use std::path::Path;

// use crate::systems::debug::ENABLE_INSPECTOR;
use crate::components::textures::GraphicsHandles;
use crate::components::player::Player;
// use crate::systems::fadeout::ScreenFade;
use crate::TILE_SIZE;
// use crate::GameState;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Map;

#[derive(Component, Clone, Inspectable)]
pub struct Door {
    pub path: String,
    pub new_x: i32,
    pub new_y: i32,
}

//TODO add direction from collision
#[derive(Clone, Inspectable)]
pub struct ExitEvent(pub Door);

#[derive(Component)]
pub struct TileCollider;

#[derive(Component)]
//TODO needs some stats
pub struct WildSpawn;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ExitEvent>()
            .add_startup_system(draw_map);
    }
}

fn draw_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    // asset_server.watch_for_changes().unwrap();
    
    let ldtk_handle = asset_server.load("ldtk/WorldMap_Free_layout.ldtk");
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
}
