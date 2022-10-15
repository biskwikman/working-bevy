use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::{errors::NearlySingularConversion, orientation::Direction};
use bevy_rapier2d::prelude::*;

// use crate::components::collision::Collider;
use crate::components::player::*;
use crate::components::textures::GraphicsHandles;
// use crate::components::collision::CollisionEvent;
use crate::systems::input::default_input_map;
use crate::systems::graphics::YSort;
use crate::TILE_SIZE;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(bevy::input::InputPlugin)
            .add_plugin(InputManagerPlugin::<Action>::default())
            .add_startup_system(spawn_player)
            .add_event::<PlayerWalk>()
            .add_system(player_walks)
            .add_system(move_player.after(player_walks))
            .add_system(face_player.after(player_walks))
            .add_system(set_player_is_moving.after(move_player));
    }
}

fn move_player(
    mut player_query: Query<(&Player, &mut Transform)>,
    mut event_reader: EventReader<PlayerWalk>,
    time: Res<Time>,
) {
    let (player, mut transform) = player_query.single_mut();
    for ev in event_reader.iter() {
        transform.translation.x -= ev.direction.unit_vector()[0] * player.speed * TILE_SIZE * time.delta_seconds();
        transform.translation.y += ev.direction.unit_vector()[1] * player.speed * TILE_SIZE * time.delta_seconds();
    }
}

fn face_player(
    mut event_reader: EventReader<PlayerWalk>,
    mut player_query: Query<&mut Player>,
) {
    let mut player = player_query.single_mut();

    for ev in event_reader.iter() {
        let x = ev.direction.unit_vector()[0];
        let y = ev.direction.unit_vector()[1];

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

fn set_player_is_moving(mut player_query: Query<(&mut Player, ChangeTrackers<Transform>)>)
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
    let sprite = TextureAtlasSprite::new(animations.walk_down[0]);
    // sprite.custom_size = Some(Vec2::new(0.6, 0.9));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: graphics.characters.clone(),
            transform: Transform {
                translation: Vec3::new(-30.0, 0.0, 900.0),
                ..default()
            },
            ..default()
    })
        .insert(Name::new("Player"))
        .insert(Player {
            speed: PLAYER_SPEED,
            current_direction: FacingDirection::Down,
            hitbox_size: 32.0,
            is_moving: false,
            just_moved: false,
            active: true,
        })
        .insert_bundle(InputManagerBundle {
            input_map: default_input_map(),
            action_state: ActionState::default(),
        })
        .insert(AnimatedSprite {
            current_frame: 0,
            timer: (Timer::from_seconds(0.1, true))
        })
        .insert(YSort(300.0))
        // .insert(Collider)
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(16.0, 16.0))
        .insert(Restitution {coefficient: 0.0, combine_rule: CoefficientCombineRule::Average})
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
