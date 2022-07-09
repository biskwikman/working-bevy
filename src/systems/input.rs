use bevy::{prelude::*};
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::{orientation::Direction};

use crate::components::player::*;

pub struct InputPlugin;

impl Action {
    pub const DIRECTIONS: [Self; 4] = [
        Action::Up,
        Action::Down,
        Action::Left,
        Action::Right,
    ];

    pub fn direction(self) -> Option<Direction> {
        match self {
            Action::Up => Some(Direction::NORTH),
            Action::Down => Some(Direction::SOUTH),
            Action::Left => Some(Direction::EAST),
            Action::Right => Some(Direction::WEST),
            _ => None,
        }
    }
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(bevy::input::InputPlugin)
            .add_plugin(InputManagerPlugin::<Action>::default());
    }
}
