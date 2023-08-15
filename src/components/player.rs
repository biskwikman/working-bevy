use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::orientation::Direction;

pub const PLAYER_SPEED: f32 = 200.0;

#[derive(Component, Resource, Debug)]
pub struct Player {
    pub speed: f32,
    pub current_direction: FacingDirection,
    pub is_moving: bool,
    pub hitbox_size: f32,
    pub just_moved: bool,
    pub active: bool,
}

#[derive(Resource)]
pub struct PlayerAnimations {
    pub walk_up: Vec<usize>,
    pub walk_down: Vec<usize>,
    pub walk_left: Vec<usize>,
    pub walk_right: Vec<usize>,
    pub face_up: Vec<usize>,
    pub face_down: Vec<usize>,
    pub face_left: Vec<usize>,
    pub face_right: Vec<usize>,
}

#[derive(Debug)]
pub enum FacingDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct AnimatedSprite {
    pub current_frame: usize,
    pub timer: Timer,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Reflect, Debug)]
pub enum Action {
    // Movement
    Up,
    Down,
    Left,
    Right,
    // Abilities
    Interact,
}

#[derive(Event)]
pub struct PlayerWalk {
    pub direction: Direction
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub input_manager: InputManagerBundle<Action>
}
