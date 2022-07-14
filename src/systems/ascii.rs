use bevy::prelude::*;

use crate::TILE_SIZE;

#[derive(Clone)]
pub struct AsciiSheet(pub Handle<TextureAtlas>);

pub struct AsciiPlugin;

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_ascii);
    }
}

pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    textures: &AsciiSheet,
    index: usize,
    color: Color,
    translation: Vec3,
    scale: Vec3,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE * 2.0));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: textures.0.clone(),
            transform: Transform{ 
                translation: translation,
                ..default()
            },
            ..default()
        }).id()
}

fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // let char_image = assets.load("textures/main-char-sheet.png");
    let ascii_image = assets.load("textures/ascii-sheet.png");

    // let char_atlas = TextureAtlas::from_grid(
    //     char_image,
    //     Vec2::new(32.0, 48.0),
    //     10,
    //     1,
    // );

    let ascii_atlas = TextureAtlas::from_grid(
        ascii_image,
        Vec2::new(16.0, 16.0),
        16,
        16,
    );

    // let char_atlas_handle = texture_atlases.add(char_atlas);
    let ascii_atlas_handle = texture_atlases.add(ascii_atlas);

    // commands.insert_resource(CharSheet(char_atlas_handle));
    commands.insert_resource(AsciiSheet(ascii_atlas_handle));
}