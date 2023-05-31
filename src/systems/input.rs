use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::orientation::Direction;

use crate::components::player::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugin(bevy::input::InputPlugin)
            .add_plugin(InputManagerPlugin::<Action>::default());
    }
}

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

impl PlayerBundle {
    pub fn default_input_map() -> InputMap<Action> {
        use Action::*;
        let mut input_map = InputMap::default();

        // This is a quick and hacky solution:
        // you should coordinate with the `Gamepads` resource to determine the correct gamepad for each player
        // and gracefully handle disconnects
        // input_map.set_gamepad(Gamepad(0));

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
