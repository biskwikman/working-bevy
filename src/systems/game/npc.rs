use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::textures::GraphicsHandles;
use crate::components::npc::Npc;
use crate::systems::game::graphics::YSort;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_npc);
    }
}

fn spawn_npc (
    mut commands: Commands,
    graphics: Res<GraphicsHandles>,
) {
    commands
        .spawn(SpriteBundle {
            texture: graphics.npcs.clone(),
            transform: Transform {
                translation: Vec3::new(300.0, 40.0, 900.0),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Talker"))
        .insert(Npc)
        .insert(YSort(300.0))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(14.0, 14.0))
        .insert(GravityScale(0.0));
}
