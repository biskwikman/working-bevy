use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::systems::game::YSort;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Door;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct DoorBundle {
    #[sprite_sheet_bundle(no_grid)]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub door: Door,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub ysort: YSort,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct VendingMachine;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct VendingMachineBundle {
    #[sprite_sheet_bundle(no_grid)]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub vending_machine: VendingMachine,
    #[from_entity_instance]
    pub ysort: YSort,
}

#[derive(Clone, Debug, Default, Bundle)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub gravity_scale: GravityScale,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> Self {
        match entity_instance.identifier.as_ref() {
            "Door" => ColliderBundle {
                collider: Collider::cuboid(16.0, 24.0),
                rigid_body: RigidBody::Fixed,
                gravity_scale: GravityScale(0.0),
            },
            _ => ColliderBundle::default(),
        }
    }
}

impl From<&EntityInstance> for YSort {
    fn from(entity_instance: &EntityInstance) -> Self {
        match entity_instance.identifier.as_ref() {
            "VendingMachine" => YSort(300.0),
            "Door" => YSort(300.0),
            _ => YSort::default(),
        }
    }
}
