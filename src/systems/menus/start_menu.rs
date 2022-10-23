use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_menu);
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(ButtonBundle {
        style: Style {
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            size: Size::new(Val::Percent(20.0), Val::Percent(10.0)),
            margin: UiRect::all(Val::Auto),
            ..default()
        },
        color: NORMAL_BUTTON.into(),
        ..default()
    })
    .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Button",
                TextStyle { 
                    font: asset_server.load("fonts/DungeonFont.ttf"),
                    font_size: 30.0,
                    color: Color::rgb(255.0, 255.0, 255.0)
                }
            ));
        });
}
