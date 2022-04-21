use bevy::{prelude::*};

use crate::CharSheet;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system(player_movement);
    }
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
) {
    // a pattern matching let
    let (_player, mut transform) = player_query.single_mut();

    if keyboard.pressed(KeyCode::W) {
        transform.translation.y += 0.5;
    }
    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= 0.5;
    }
    if keyboard.pressed(KeyCode::A) {
        transform.translation.x -= 0.5;
    }
    if keyboard.pressed(KeyCode::D) {
        transform.translation.x  += 0.5;
    }
}

fn spawn_player(mut commands: Commands, character: Res<CharSheet>){
    let mut sprite = TextureAtlasSprite::new(0);
    // of x and y are both 1, sprite is squashed
    sprite.custom_size = Some(Vec2::new(1.0, 2.0));

    commands.spawn_bundle(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: character.0.clone(),
        transform: Transform{ 
            translation: Vec3::new(0.0, 0.0, 900.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Name::new("Player"))
    .insert(Player);
}