use std::{fs::File, io::{BufReader, BufRead}};
use bevy::prelude::*;

use crate::{textures::{spawn_textures_sprite, AsciiSheet}, TILE_SIZE};

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_simple_map);
    }
}

fn create_simple_map(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let file = File::open("assets/maps/simple_map.txt").expect("No map file found");

    for(y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                spawn_textures_sprite(
                    &mut commands,
                    &ascii,
                    char as usize,
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0)
                );
            }
        }
    }
}