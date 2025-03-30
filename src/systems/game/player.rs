use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
// use leafwing_input_manager::{errors::NearlySingularConversion, orientation::Direction};
use bevy_rapier2d::prelude::*;

use crate::components::overworld::OverworldState;
use crate::components::player::*;
use crate::components::textures::GraphicsHandles;
use crate::systems::game::graphics::YSort;
use crate::components::global::{TILE_SIZE, GameState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_event::<PlayerWalk>()
            .add_systems(Update, 
                (
                    player_walks,
                    move_player,
                    face_player,
                    set_player_is_moving,
                ).chain()
                .run_if(in_state(GameState::InGame))
                .run_if(in_state(OverworldState::FreeRoam))
            );
    }
}

fn move_player(
    mut player_query: Query<(&Player, &mut Transform)>,
    mut event_reader: EventReader<PlayerWalk>,
    time: Res<Time>,
) {
    let (player, mut transform) = player_query.single_mut();
    for ev in event_reader.read() {
        // transform.translation.x -= ev.direction.unit_vector()[0] * player.speed * TILE_SIZE * time.delta_secs();
        // transform.translation.y += ev.direction.unit_vector()[1] * player.speed * TILE_SIZE * time.delta_secs();
        transform.translation.x -= ev.direction.cos * player.speed * TILE_SIZE * time.delta_secs();
        transform.translation.y += ev.direction.sin * player.speed * TILE_SIZE * time.delta_secs();
    }
}

fn face_player(
    mut event_reader: EventReader<PlayerWalk>,
    mut player_query: Query<&mut Player>,
) {
    let mut player = player_query.single_mut();

    for ev in event_reader.read() {
        // let x = ev.direction.unit_vector()[0];
        // let y = ev.direction.unit_vector()[1];
        let x = ev.direction.cos;
        let y = ev.direction.sin;

        if y > 0.0 && x == 0.0 {
            player.current_direction = FacingDirection::Up;
        }
        if y < 0.0 && x == 0.0 {
            player.current_direction = FacingDirection::Down;
        }
        if x < 0.0 {
            player.current_direction = FacingDirection::Left;
        }
        if x > 0.0 {
            player.current_direction = FacingDirection::Right;
        }
    }
}

fn set_player_is_moving(mut player_query: Query<(&mut Player, Ref<Transform>)>)
    {
    for (mut player, trackers) in player_query.iter_mut() {
        if trackers.is_changed() {
            player.is_moving = true;
        } else {
            player.is_moving = false;
        }
    }
}

fn spawn_player(
    mut commands: Commands, 
    graphics: Res<GraphicsHandles>,
    animations: Res<PlayerAnimations>, 
) {
    // let sprite = TextureAtlas::new(animations.walk_down[0]);

    commands
        .spawn(Sprite {
            image: graphics.character.image,
            texture_atlas: Some(TextureAtlas {layout: graphics.character.texture_atlas_layout, index: animations.face_down[0]}),
            // sprite,
            // texture_atlas: graphics.characters.clone(),
            // transform: Transform {
            //     translation: Vec3::new(70.0, 40.0, 900.0),
            //     ..default()
            // },
            // ..default()
        })
        .insert(Name::new("Player"))
        .insert(PlayerBundle {
            player: Player {
                speed: PLAYER_SPEED,
                current_direction: FacingDirection::Down,
                hitbox_size: 32.0,
                is_moving: false,
                just_moved: false,
                active: true,
            },
            input_manager: InputManagerBundle {
                input_map: PlayerBundle::default_input_map(),
                ..default()
            }
        })
        .insert(AnimatedSprite {
            current_frame: 0,
            timer: (Timer::from_seconds(0.1, TimerMode::Repeating))
        })
        .insert(YSort(300.0))
        .insert(RigidBody::Dynamic)
        .with_children(|children| {
            children.spawn(Collider::cuboid(15.0, 8.0))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, -16.0, 0.0)));
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(GravityScale(0.0))
        .insert(LockedAxes::ROTATION_LOCKED);
}

fn player_walks(
    query: Query<&ActionState<Action>, With<Player>>,
    mut event_writer: EventWriter<PlayerWalk>,
) {
    let action_state = query.single();
    
    let mut direction_vector = Vec2::ZERO;

    for input_direction in Action::DIRECTIONS {
        if action_state.pressed(input_direction) {
            if let Some(direction) = input_direction.direction() {
                direction_vector += Vec2::from(direction);
            }
        }
    }
    // Normalize magnitude
    let net_direction: Result<Direction, NearlySingularConversion> = direction_vector.try_into();

    if let Ok(direction) = net_direction {
            event_writer.send(PlayerWalk {direction});
    }
}
