use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::maps::{Wall, WallBundle};

pub struct MapsPlugin;

impl Plugin for MapsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(LdtkPlugin)
            .add_systems(Startup, (load_map, spawn_wall_collision))
            // .add_systems(Update, spawn_wall_collision)
            .insert_resource(LevelSelection::Index(0))
            .register_ldtk_int_cell::<WallBundle>(1);
    }
}

fn load_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/working.ldtk"),
        ..Default::default()
    });
}

fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<(&Wall, Entity)>,
) {
    for (_wall, ent) in wall_query.iter() {
        commands.spawn(Collider::cuboid(16.0, 16.0));
        commands.entity(ent).insert(Collider::cuboid(16.0, 16.0));
    }

}
