use bevy::prelude::*;
use bevy_inspector_egui::{WorldInspectorPlugin, RegisterInspectable};

use crate::components::player::Player;

#[cfg(debug_assertions)]
pub const ENABLE_INSPECTOR: bool = true;
#[cfg(not(debug_assertions))]
pub const ENABLE_INSPECTOR: bool = false;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<Player>();
        }
    }
}