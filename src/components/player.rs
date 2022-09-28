use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy_inspector_egui::Inspectable;
use leafwing_input_manager::orientation::Direction;

pub const PLAYER_SPEED: f32 = 5.0;

#[derive(Component, Inspectable, Debug)]
pub struct Player {
    pub speed: f32,
    pub current_direction: FacingDirection,
    pub is_moving: bool,
    pub hitbox_size: f32,
    pub just_moved: bool,
    pub active: bool,
}

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

#[derive(Debug, Inspectable)]
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

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Inspectable, Debug)]
pub enum Action {
    // Movement
    Up,
    Down,
    Left,
    Right,
    // Abilities
    Interact,
}

pub struct PlayerWalk {
    pub direction: Direction
}
