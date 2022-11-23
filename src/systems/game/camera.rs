use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use crate::components::player::Player;
use crate::components::camera::Camera;

pub const RESOLUTION: f32 = 16.0 / 9.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player_camera)
            .add_system(camera_follow_player);
    }
}

pub fn spawn_player_camera(mut commands: Commands) {
    let camera_view_factor = 50.0;
    commands
        .spawn_bundle(Camera2dBundle {
            projection: OrthographicProjection {
                top: camera_view_factor,
                bottom: -camera_view_factor,
                right: camera_view_factor * RESOLUTION,
                left: -camera_view_factor * RESOLUTION,
                scaling_mode: ScalingMode::None,
                ..default()
            },
            ..default()
        })
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
