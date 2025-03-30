use bevy::prelude::*;

#[derive(Resource)]
pub struct GraphicsHandles {
    pub character: GraphicsBundle,
    // pub character: Handle<TextureAtlasLayout>,
    pub tiles: Handle<TextureAtlasLayout>,
    pub npcs: Handle<Image>,
}

pub struct GraphicsBundle {
    pub image: Handle<Image>,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
}
