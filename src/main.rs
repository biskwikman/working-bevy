#![allow(clippy::redundant_field_names)]
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::render::camera::ScalingMode;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;

mod player;
use player::PlayerPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: 1600.0,
            height: 900.0,
            title: "Working".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(spawn_camera)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_textures)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

struct CharSheet(Handle<TextureAtlas>);

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