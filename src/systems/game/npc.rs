use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::npc::Npc;
use crate::components::textures::GraphicsHandles;
use crate::systems::game::graphics::YSort;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_npc);
    }
}

fn spawn_npc(mut commands: Commands, graphics: Res<GraphicsHandles>) {
    commands
        .spawn(SpriteBundle {
            texture: graphics.npcs.clone(),
            transform: Transform {
                translation: Vec3::new(350.0, 40.0, 900.0),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Talker"))
        .insert(Npc)
        .insert(YSort(300.0))
        .insert(RigidBody::Fixed)
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(15.0, 8.0))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, -16.0, 0.0)));
        })
        .insert(GravityScale(0.0));
}
