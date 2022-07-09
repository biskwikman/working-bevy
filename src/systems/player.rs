use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::{errors::NearlySingularConversion, orientation::Direction};

use crate::components::player::*;
// use crate::components::input::*;
use crate::components::textures::CharSheet;
use crate::TILE_SIZE;
use crate::systems::textures::spawn_textures_sprite;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(bevy::input::InputPlugin)
            .add_plugin(InputManagerPlugin::<Action>::default())
            .add_startup_system(spawn_player)
            .add_event::<PlayerWalk>()
            .add_system(player_walks)
            .add_system(player_movement.after(player_walks))
            .add_system(player_animation.after(player_walks));
    }
}

// #[derive(Bundle)]
// struct PlayerBundle {
//     player: Player,
//     #[bundle]
//     input_manager: InputManagerBundle<Action>
// }

impl PlayerBundle {
    fn default_input_map() -> InputMap<Action> {
        use Action::*;
        let mut input_map = InputMap::default();

        // This is a quick and hacky solution:
        // you should coordinate with the `Gamepads` resource to determine the correct gamepad for each player
        // and gracefully handle disconnects
        input_map.set_gamepad(Gamepad(0));

        // Movement
        input_map.insert(KeyCode::Up, Up);
        input_map.insert(KeyCode::W, Up);
        input_map.insert(GamepadButtonType::DPadUp, Up);


        input_map.insert(KeyCode::Down, Down);
        input_map.insert(KeyCode::S, Down);
        input_map.insert(GamepadButtonType::DPadDown, Down);

        input_map.insert(KeyCode::Left, Left);
        input_map.insert(KeyCode::A, Left);
        input_map.insert(GamepadButtonType::DPadLeft, Left);

        input_map.insert(KeyCode::Right, Right);
        input_map.insert(KeyCode::D, Right);
        input_map.insert(GamepadButtonType::DPadRight, Right);

        // Abilities
        input_map.insert(KeyCode::F, Interact);
        input_map.insert(GamepadButtonType::East, Interact);

        input_map
    }
}

fn player_movement(
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

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn player_animation(
    time: Res<Time>,
    mut event_reader: EventReader<PlayerWalk>,
    mut player_query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    let (
        mut timer, 
        mut sprite, 
    ) = player_query.single_mut();

    for ev in event_reader.iter() {
        let x = ev.direction.unit_vector()[0];
        let y = ev.direction.unit_vector()[1];

        //up animation
        if y > 0.0 && x == 0.0 {
            timer.tick(time.delta());
            if timer.just_finished() {
                if sprite.index != 8 {
                    sprite.index = 8;
                }
                else if sprite.index == 8 {
                    sprite.index = 9;
                }
            }
        }
        //down animation
        if y < 0.0 && x == 0.0 {
            timer.tick(time.delta());
            if timer.just_finished() {
                if sprite.index != 1 {
                    sprite.index = 1;
                }
                else if sprite.index == 1 {
                    sprite.index = 2;
                }
            }
        }
        //left animation
        if x < 0.0 {
            timer.tick(time.delta());
            if timer.just_finished() {
                if sprite.index != 5 {
                    sprite.index = 5;
                }
                else if sprite.index == 5 {
                    sprite.index = 6;
                }
            }
        }
        //right animation
        if x > 0.0 {
            timer.tick(time.delta());
            if timer.just_finished() {
                if sprite.index != 3 {
                    sprite.index = 3;
                }
                else if sprite.index == 3 {
                    sprite.index = 4;
                }
            }
        }
    }
}

fn spawn_player(mut commands: Commands, character: Res<CharSheet>) {
    let player_ent = spawn_textures_sprite(
        &mut commands,
        &character,
        0,
        Vec3::new(0.0, 0.0, 900.0),
    );

    commands
        .entity(player_ent)
        .insert_bundle(PlayerBundle {
            player: Player { speed: PLAYER_SPEED },
            input_manager: InputManagerBundle {
                input_map: PlayerBundle::default_input_map(),
                action_state: ActionState::default(),
            }
        })
        .insert(Name::new("Player"))
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
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