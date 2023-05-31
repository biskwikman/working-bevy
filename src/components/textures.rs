use bevy::prelude::*;

#[derive(Resource)]
pub struct GraphicsHandles {
    pub characters: Handle<TextureAtlas>,
    pub tiles: Handle<TextureAtlas>,
    pub npcs: Handle<Image>,
}
