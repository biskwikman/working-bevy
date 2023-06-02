use::bevy::prelude::*;

#[derive(States, PartialEq, Eq, Debug, Default, Clone, Hash)]
pub enum OverworldState {
    #[default]
    FreeRoam,
    Dialog,
    Menu,
    NotInOverworld,
}
