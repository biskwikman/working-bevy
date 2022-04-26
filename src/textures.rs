use bevy::prelude::*;

use crate::TILE_SIZE;

pub struct TexturesPlugin;

pub struct CharSheet(Handle<TextureAtlas>);

impl Plugin for TexturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_textures);
    }
}

pub fn spawn_textures_sprite(
    commands: &mut Commands,
    textures: &CharSheet,
    index: usize,
    translation: Vec3,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE * 2.0));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: textures.0.clone(),
            transform: Transform{ 
                translation: translation,
                ..Default::default()
            },
            ..Default::default()
        }).id()
}

fn load_textures(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("textures/main-char-sheet.png");
    let atlas = TextureAtlas::from_grid(
        image, 
        Vec2::new(32.0, 64.0), 
        10, 
        1, 
    );

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(CharSheet(atlas_handle));
}