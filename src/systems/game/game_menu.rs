use bevy::prelude::*;
use crate::components::overworld::OverworldState;

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugins(DefaultNavigationPlugins)
            .add_systems(Update, (spawn_menu, despawn_menu));    
        }
}

#[derive(Component)]
pub struct GameMenu;

fn despawn_menu(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    overworld_state_current: Res<State<OverworldState>>,
    mut overworld_state_next: ResMut<NextState<OverworldState>>,
    game_menus: Query<Entity, With<GameMenu>>,
) {
    if input.just_pressed(KeyCode::Escape) && overworld_state_current.get() == &OverworldState::Menu {
        for game_menu in &game_menus {
            commands.entity(game_menu).despawn();
        }
        overworld_state_next.set(OverworldState::FreeRoam);
    }
}

fn spawn_menu(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    overworld_state_current: Res<State<OverworldState>>,
    mut overworld_state_next: ResMut<NextState<OverworldState>>,
) {
    if !input.just_pressed(KeyCode::Escape) {
        return;
    }

    if overworld_state_current.get() == &OverworldState::FreeRoam {
        let (background, _buttons) = build_menu();
        commands.spawn((background, GameMenu));
        overworld_state_next.set(OverworldState::Menu)
    }
}

fn build_menu() -> (Node, Button) {
    let menu_background = 
        Node {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            // background_color: Color::BLUE.into(),
            ..Default::default()
        };

    let buttons = 
        Button {
            // height: Val::Px(95.0),
            // width: Val::Px(65.0),
            // position: UiRect::left(Val::Percent(30.0)),
            // position_type: PositionType::Absolute,
            // background_color: Color::DARK_GRAY.into(),
            ..Default::default()
        };
    (menu_background, buttons)
}
