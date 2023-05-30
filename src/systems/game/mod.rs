use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub mod camera;
pub mod dialog_box;
pub mod graphics;
pub mod npc;
pub mod player;

pub use camera::*;
pub use dialog_box::*;
pub use graphics::*;
pub use npc::*;
pub use player::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
       app
            .insert_resource(LevelSelection::Index(0))
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(CameraPlugin)
            .add_plugin(DialogBoxPlugin)
            .add_plugin(GraphicsPlugin)
            .add_plugin(NpcPlugin)
            .add_plugin(PlayerPlugin);
    }
}
