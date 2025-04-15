use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::components::player::Action;
use crate::components::global::GameState;
use crate::components::player::Player;
use crate::components::npc::Npc;
use crate::components::overworld::OverworldState;
use crate::components::dialog_box::*;

pub struct DialogBoxPlugin;

impl Plugin for DialogBoxPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update,
                (
                    talk,
                    despawn_dialog_box
                )
                .run_if(in_state(GameState::InGame))
            );
    }
}

fn spawn_dialog_box(
    commands: &mut Commands,
    assets: &Res<AssetServer>,
    starting_text: &str,
) -> Entity {
    let font = assets.load("fonts/DungeonFont.ttf");

    let parent_node = (
        Node {
            width: Val::Percent(80.0), 
            height: Val::Percent(30.0),
            align_self: AlignSelf::FlexEnd,
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Row,
            position_type: PositionType::Absolute,
            left: Val::Percent(10.0),
            margin: UiRect::bottom(Val::Percent(4.0)),
            padding: UiRect::new(Val::Percent(1.0), Val::Auto, Val::Px(15.0), Val::Auto),
            ..default()
        },
        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
        DialogUI,
        Name::new("Dialog UI")
    );

    let dialog_text = (
        Text::new(starting_text),
        TextFont {font, font_size: 36.0, ..default()},
        TextLayout::new_with_justify(JustifyText::Left),
        NpcText,
    );

    commands
        .spawn(parent_node)
        .with_children(|commands| {
            commands.spawn(dialog_text);
        })
        .id()
}

fn despawn_dialog_box(
    mut commands: Commands,
    // input: Res<Input<KeyCode>>,
    input: Query<&ActionState<Action>, With<Player>>,
    dialog: Query<Entity, With<DialogUI>>,
    mut overworld_state: ResMut<NextState<OverworldState>>,
    overworld_state_current: Res<State<OverworldState>>,
    
) {
    let action_state = input.single();
    if action_state.just_pressed(&Action::Interact) && overworld_state_current.get() == &OverworldState::Dialog {
        for dialog in &dialog {
            println!("{:?}", dialog);
            commands.entity(dialog).despawn_recursive();
            overworld_state.set(OverworldState::FreeRoam);
        }
    }
}

fn talk(
    mut commands: Commands,
    assets: Res<AssetServer>,
    player: Query<&Transform, With<Player>>,
    npcs: Query<(&Transform, Entity, &Npc)>,
    input: Query<&ActionState<Action>, With<Player>>,
    // input: Res<Input<KeyCode>>,
    mut overworld_state: ResMut<NextState<OverworldState>>,
    overworld_state_current: Res<State<OverworldState>>,
) {
    let action_state = input.single();
    let player = player.single();

    if !action_state.just_pressed(&Action::Interact) {
        return;
    }
    
    for (npc, _entity, _id) in &npcs {
        if Vec2::distance(player.translation.truncate(), npc.translation.truncate()) < 40.0 
            && overworld_state_current.get() == &OverworldState::FreeRoam {
                overworld_state.set(OverworldState::Dialog);
                spawn_dialog_box(
                    &mut commands, 
                    &assets,
                    "Hello",
                );
        }
    }
}
