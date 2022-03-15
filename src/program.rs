use bevy::prelude::*;

const FONT_PATH: &str = "font.otf";

#[derive(Component)]
struct FgText;
#[derive(Component)]
struct BgText;

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
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Placeholder".to_string(),
                        TextStyle {
                            font,
                            font_size: 64.0,
                            color: Color::GRAY,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
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

pub struct ProgramPlugin;
impl Plugin for ProgramPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_startup_system(setup_bg_text)
            .add_startup_system(setup_fg_text);
    }
}
