use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub mod camera;
pub mod dialog_box;
pub mod graphics;
pub mod npc;
pub mod player;
pub mod game_menu;
pub mod maps;

pub use camera::*;
pub use dialog_box::*;
pub use graphics::*;
pub use npc::*;
pub use player::*;
pub use game_menu::*;
pub use maps::*;

use crate::components::overworld::OverworldState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
       app
            .insert_resource(LevelSelection::Index(0))
            .add_state::<OverworldState>()
            .add_plugins((RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0), 
                            CameraPlugin, 
                            DialogBoxPlugin, 
                            GraphicsPlugin, 
                            NpcPlugin, 
                            PlayerPlugin, 
                            GameMenuPlugin,
                            MapsPlugin,
                        ));
    }
}
