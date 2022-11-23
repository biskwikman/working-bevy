use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// use crate::components::collision::Collider;
use crate::components::textures::GraphicsHandles;
use crate::components::npc::Npc;
use crate::systems::graphics::YSort;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_npc);
    }
}

fn spawn_npc (
    mut commands: Commands,
    graphics: Res<GraphicsHandles>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: graphics.npcs.clone(),
            transform: Transform {
                translation: Vec3::new(5.0, 0.0, 900.0),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Talker"))
        .insert(Npc)
        .insert(YSort(300.0))
        // .insert(Collider)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(14.0, 14.0))
        .insert(GravityScale(0.0));
}
