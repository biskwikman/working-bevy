use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use crate::components::player::Player;

use crate::components::collision::Collider;

pub struct CollidePlugin;

impl Plugin for CollidePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(check_player_collision);
    }
}

fn check_player_collision (
    mut collidee_query: Query<&Transform, (With<Collider>, Without<Player>)>,
    player_query: Query<(&Player, &Transform), With<Collider>>,
) {
    let collidee_transform = collidee_query.single_mut();
    for (player, player_transform) in &player_query {
        let collision = collide(
            collidee_transform.translation,
            collidee_transform.scale.truncate(),
            player_transform.translation,
            player_transform.scale.truncate(),
        );
    }

    // if let Some(_) = collision {
    //     eprint!("collision");
    // }
}
