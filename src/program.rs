use bevy::prelude::*;

const FONT_PATH: &str = "font.otf";

#[derive(Component)]
pub struct FgText;
#[derive(Component)]
struct BgText;

pub struct CurrentKey(pub Option<char>);

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup_bg_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(FONT_PATH);

    commands
        .spawn_bundle(NodeBundle {
            color: Color::NONE.into(),
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_self: AlignSelf::FlexEnd,
                        margin: Rect {
                            top: Val::Percent(20.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Placeholder".to_string(),
                        TextStyle {
                            font,
                            font_size: 64.0,
                            color: Color::SEA_GREEN,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Top,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                })
                .insert(BgText);
        });
}

fn setup_fg_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(FONT_PATH);

    commands
        .spawn_bundle(NodeBundle {
            color: Color::NONE.into(),
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "".to_string(),
                        TextStyle {
                            font,
                            font_size: 64.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                })
                .insert(FgText);
        });
}

fn handle_char(current_char: Res<CurrentKey>, mut q_fgtext: Query<&mut Text, With<FgText>>) {
    if current_char.is_changed() {
        let mut fgtext = q_fgtext.get_single_mut().unwrap();
        if current_char.0.is_some() {
            if current_char.0.unwrap() == ' ' {
                // TODO
            }

            fgtext.sections[0].value.push(current_char.0.unwrap());
        }
    }
}

pub struct ProgramPlugin;
impl Plugin for ProgramPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentKey(None))
            .add_startup_system(setup_camera)
            .add_startup_system(setup_bg_text)
            .add_startup_system(setup_fg_text)
            .add_system(handle_char);
    }
}
