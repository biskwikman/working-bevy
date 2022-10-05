use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub const RESOLUTION: f32 = 16.0 / 9.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {

}

#[derive(Component)]
struct PlayerCamera;

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
        .insert(PlayerCamera);
}

pub fn camera_follow_player(mut camera_query: Query<(&PlayerCamera, &mut Transform)>) {
    for (camera, tranform) in camera_query.iter_mut() {

    }
}
