use bevy::prelude::*;
// use bevy::utils::tracing::instrument::WithSubscriber;
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

        // Movement
        input_map.insert(Up, KeyCode::ArrowUp);
        input_map.insert(Up, KeyCode::KeyW);
        input_map.insert(Up, GamepadButton::DPadUp);

        input_map.insert(Down, KeyCode::ArrowDown);
        input_map.insert(Down, KeyCode::KeyS);
        input_map.insert(Down, GamepadButton::DPadDown);

        input_map.insert(Left, KeyCode::ArrowLeft);
        input_map.insert(Left, KeyCode::KeyA);
        input_map.insert(Left, GamepadButton::DPadLeft);

        input_map.insert(Right, KeyCode::ArrowRight);
        input_map.insert(Right, KeyCode::KeyD);
        input_map.insert(Right, GamepadButton::DPadRight);

        // Abilities
        input_map.insert(Interact, KeyCode::KeyF);
        input_map.insert(Interact, GamepadButton::East);

        input_map
    }
}
