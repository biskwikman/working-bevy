use bevy::prelude::*;

use crate::components::textures::GraphicsHandles;

#[derive(Component, Debug)]
pub struct Npc;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

fn spawn_npc (
    mut commands: Commands,
    graphics: GraphicsHandles, 
) {
    let mut sprite = TextureAtlasSprite { color: (), index: (), flip_x: (), flip_y: (), custom_size: (), anchor: () };
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
        })
}
