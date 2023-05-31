use::bevy::prelude::*;

pub const TILE_SIZE: f32 = 0.2;
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
}
