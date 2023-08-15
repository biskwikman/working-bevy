use bevy::prelude::*;
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
        NodeBundle {
            style: Style {
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
            background_color: BackgroundColor(Color::GRAY),
            ..default()
        },
        DialogUI,
        Name::new("Dialog UI")
    );

    let dialog_text = (
        TextBundle::from_section(
            starting_text,
            TextStyle { 
                font: font, 
                font_size: 36.0, 
                color: Color::BLACK 
            },
        ).with_text_alignment(TextAlignment::Left),
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
    input: Res<Input<KeyCode>>,
    dialog: Query<Entity, With<DialogUI>>,
    mut overworld_state: ResMut<NextState<OverworldState>>,
    overworld_state_current: Res<State<OverworldState>>,
    
) {
    if input.just_pressed(KeyCode::E) && overworld_state_current.get() == &OverworldState::Dialog {
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
    input: Res<Input<KeyCode>>,
    mut overworld_state: ResMut<NextState<OverworldState>>,
    overworld_state_current: Res<State<OverworldState>>,
) {
    let player = player.single();

    if !input.just_pressed(KeyCode::E) {
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
