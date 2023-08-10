use bevy::prelude::*;
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
        .spawn(Camera2dBundle {
            projection: OrthographicProjection {
                area: Rect::new(-camera_view_factor * RESOLUTION, -camera_view_factor, camera_view_factor * RESOLUTION, camera_view_factor,),
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
