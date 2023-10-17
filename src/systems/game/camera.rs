use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use crate::components::player::Player;
use crate::components::camera::Camera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player_camera)
            .add_systems(Update, camera_follow_player);
    }
}

pub fn spawn_player_camera(mut commands: Commands) {

    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    camera.projection.scale = 1.5;
    commands
        .spawn(camera)
        .insert(Name::new("PlayerCamera"))
        .insert(Camera);
}

pub fn camera_follow_player(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    mut player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let player_transform = player_query.single_mut();
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.y = player_transform.translation.y;
    camera_transform.translation.x = player_transform.translation.x;
}
