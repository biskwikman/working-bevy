use bevy::prelude::*;
use crate::components::player::{
    AnimatedSprite, FacingDirection, Player, PlayerAnimations, PlayerWalk,
};
use crate::components::textures::{GraphicsBundle, GraphicsHandles};

// use crate::systems::debug::ENABLE_INSPECTOR;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_graphics)
            .add_event::<PlayerWalk>()
            .add_systems(Update, (animate_player, animate_sprites, y_sort));
    }
}

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct YSort(pub f32);

pub fn y_sort(mut query: Query<(&mut Transform, &YSort)>) {
    for (mut transform, ysort) in query.iter_mut() {
        transform.translation.z = ysort.0 - transform.translation.y;
    }
}

fn load_graphics(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load main char sheet
    let main_char_sheet = assets.load("textures/main-char-sheet.png");
    let main_char_atlas_layout = TextureAtlasLayout::from_grid(
        UVec2::new(32, 48),
        10,
        1,
        Some(UVec2::splat(0)),
        None,
    );
    let character_handle = atlases.add(main_char_atlas_layout);

    // Load Npc sheet
    let npc_image = assets.load("textures/talker-front.png");

    let tile_image: Handle<Image> = assets.load("tiles/basictiles.png");
    let atlas_layout = TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        8,
        15,
        Some(UVec2::splat(2)),
        None,
    );
    let tile_handle = atlases.add(atlas_layout);

    commands.insert_resource(GraphicsHandles {
        // character: character_handle,
        character: GraphicsBundle {image: main_char_sheet, texture_atlas_layout: character_handle},
        npcs: npc_image,
        tiles: tile_handle,
    });

    commands.insert_resource(PlayerAnimations {
        walk_down: vec![1, 2],
        walk_up: vec![8, 9],
        walk_left: vec![5, 6],
        walk_right: vec![3, 4],
        face_up: vec![7],
        face_down: vec![0],
        face_left: vec![5],
        face_right: vec![3],
    });
}

// TODO restructure this to support animations more generally
fn animate_sprites(mut sprites: Query<&mut AnimatedSprite>, time: Res<Time>) {
    for mut sprite in sprites.iter_mut() {
        sprite.timer.tick(time.delta());
        if sprite.timer.just_finished() {
            //Probs not dangerous but
            //FIXME handle wrap around
            sprite.current_frame += 1;
        }
    }
}

fn animate_player(
    mut player_query: Query<(&mut TextureAtlasSprite, &AnimatedSprite, &Player)>,
    animations: Res<PlayerAnimations>,
    mut event_reader: EventReader<PlayerWalk>,
) {
    let (mut sprite, animated_sprite, player) = player_query.single_mut();
    let current_frame = animated_sprite.current_frame;
    let mut ev = event_reader.iter().peekable();
    if ev.peek().is_some() {
        match player.current_direction {
            FacingDirection::Up => {
                if player.is_moving {
                    sprite.index = animations.walk_up[current_frame % animations.walk_up.len()];
                } else {
                    sprite.index = animations.face_up[0];
                }
            }
            FacingDirection::Down => {
                if player.is_moving {
                    sprite.index = animations.walk_down[current_frame % animations.walk_down.len()];
                } else {
                    sprite.index = animations.face_down[0];
                }
            }
            FacingDirection::Left => {
                if player.is_moving {
                    sprite.index = animations.walk_left[current_frame % animations.walk_left.len()];
                } else {
                    sprite.index = animations.face_left[0];
                }
            }
            FacingDirection::Right => {
                if player.is_moving {
                    sprite.index =
                        animations.walk_right[current_frame % animations.walk_right.len()];
                } else {
                    sprite.index = animations.face_right[0];
                }
            }
        }
    }
}
