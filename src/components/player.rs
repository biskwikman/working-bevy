use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy_inspector_egui::Inspectable;
use leafwing_input_manager::orientation::Direction;

pub const PLAYER_SPEED: f32 = 5.0;

#[derive(Component, Inspectable, Debug)]
pub struct Player {
    pub speed: f32
}

#[derive(Actionlike, PartialEq, Clone, Copy, Inspectable, Debug)]
pub enum Action {
    // Movement
    Up,
    Down,
    Left,
    Right,
    // Abilities
    Interact,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    #[bundle]
    pub input_manager: InputManagerBundle<Action>
}

pub struct PlayerWalk {
    pub direction: Direction
}