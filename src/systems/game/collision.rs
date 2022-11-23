use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use crate::components::player::Player;
use crate::components::collision::{Collider, CollisionEvent};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CollisionEvent>()
            .add_system(check_player_collision);
    }
}

fn check_player_collision (
    mut collidee_query: Query<&Transform, (With<Collider>, Without<Player>)>,
    player_query: Query<(&Player, &Transform), With<Collider>>,
    mut collision_event: EventWriter<CollisionEvent>,
) {
    let collidee_transform = collidee_query.single_mut();
    for (_player, player_transform) in &player_query {
        let collision = collide(
            collidee_transform.translation,
            collidee_transform.scale.truncate(),
            player_transform.translation,
            player_transform.scale.truncate(),
        );
        if collision.is_some() {
            collision_event.send_default();

        }
    }
}
