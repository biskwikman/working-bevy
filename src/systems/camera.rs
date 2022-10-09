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
    commands
        .spawn_bundle(Camera2dBundle {
            projection: OrthographicProjection {
                top: 1.0,
                bottom: -1.0,
                right: 1.0 * RESOLUTION,
                left: -1.0 * RESOLUTION,
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
