use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent;
use crate::components::player::Player;

pub struct DialogBoxPlugin;

impl Plugin for DialogBoxPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(check_player_collision);
    }
}

fn spawn_dialog_box() {}

fn despawn_dialog_box() {}

fn build_dialog_box(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let dialog_box_entity = commands
        .spawn(NodeBundle {
            background_color: Color::RED.into(),
            ..default()
        })
        .id();

    dialog_box_entity
}


fn check_player_collision (
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<Entity, &Player>,
) {
    for collision_event in collision_events.iter() {
        println!("Recieved Collision Event: {:?}", collision_event);
        for entity in &player_query {
            println!("Entity {:?}", entity)
        }
    }
}
