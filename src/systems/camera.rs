use bevy::prelude::*;
use bevy::render::camera::ScalingMode; 

pub const RESOLUTION: f32 = 16.0 / 9.0;

pub fn spawn_camera(mut commands: Commands) {
    // let mut camera = Camera2dBundle::default();

    // camera.orthographic_projection.top = 1.0;
    // camera.orthographic_projection.bottom = -1.0;

    // camera.orthographic_projection.right = 1.0 * RESOLUTION;
    // camera.orthographic_projection.left = -1.0 * RESOLUTION;

    // camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            top: 1.0,
            bottom: -1.0,
            right: 1.0 * RESOLUTION,
            left: -1.0 * RESOLUTION,
            scaling_mode: ScalingMode::None,
            ..default()
        }.into(),
        ..default()
    });
}