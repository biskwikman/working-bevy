use bevy::prelude::*;
use bevy::utils::tracing::instrument::WithSubscriber;
// use leafwing_input_manager::orientation::Direction;
use bevy::math::Rot2;
use leafwing_input_manager::prelude::*;

use crate::components::player::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugin(bevy::input::InputPlugin)
            .add_plugins(InputManagerPlugin::<Action>::default());
    }
}

impl Action {
    pub const DIRECTIONS: [Self; 4] = [Action::Up, Action::Down, Action::Left, Action::Right];

    pub fn direction(self) -> Option<Rot2> {
        match self {
            Action::Up => Some(Rot2::degrees(90.0)),
            Action::Down => Some(Rot2::degrees(270.0)),
            Action::Left => Some(Rot2::degrees(0.0)),
            Action::Right => Some(Rot2::degrees(180.0)),
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
        input_map.insert(Up, KeyCode::ArrowUp);
        input_map.insert(Up, KeyCode::KeyW);
        input_map.insert(Up, GamepadButton::DPadUp);

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
