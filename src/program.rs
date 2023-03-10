use crate::{sfx::ClickEvent, words::Words};
use bevy::{prelude::*, window::WindowResized};

const FONT_PATH: &str = "font.otf";
const FONT_SIZE_MULTIPLIER: f32 = 0.094;

#[derive(Component)]
pub struct FgText;
#[derive(Component)]
struct BgText;
#[derive(Component)]
struct NextText;

pub struct CurrentKey(pub Option<char>);
struct NextWord(String);

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup_bg_text(mut commands: Commands, asset_server: Res<AssetServer>, words: Res<Words>) {
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
                            top: Val::Percent(15.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        words.get_new_word(),
                        TextStyle {
                            font,
                            font_size: 128.0,
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
                            font_size: 128.0,
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

fn setup_next_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    words: Res<Words>,
    mut next_text: ResMut<NextWord>,
) {
    let font = asset_server.load(FONT_PATH);
    let word = words.get_new_word();
    next_text.0 = word.clone();

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
                            top: Val::Percent(8.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        word,
                        TextStyle {
                            font,
                            font_size: 128.0,
                            color: Color::DARK_GRAY,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Top,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                })
                .insert(NextText);
        });
}

fn handle_char(
    current_char: Res<CurrentKey>,
    mut q_fgtext: Query<&mut Text, With<FgText>>,
    mut q_bgtext: Query<&mut Text, (With<BgText>, Without<FgText>)>,
    mut q_nexttext: Query<&mut Text, (With<NextText>, Without<FgText>, Without<BgText>)>,
    words: Res<Words>,
    mut next_word: ResMut<NextWord>,
    mut ev_click: EventWriter<ClickEvent>,
) {
    if current_char.is_changed() && current_char.0.is_some() {
        ev_click.send(ClickEvent);

        let mut fgtext = q_fgtext.get_single_mut().unwrap();
        let mut bgtext = q_bgtext.get_single_mut().unwrap();
        let mut nexttext = q_nexttext.get_single_mut().unwrap();

        fgtext.sections[0].value.push(current_char.0.unwrap());

        if fgtext.sections[0].value.trim() == bgtext.sections[0].value.trim()
            || (current_char.0.unwrap() == ' '
                && fgtext.sections[0].value.len() > bgtext.sections[0].value.len() / 2)
        {
            fgtext.sections[0].value = String::from("");
            bgtext.sections[0].value = next_word.0.clone();
            next_word.0 = words.get_new_word();
            nexttext.sections[0].value = next_word.0.clone();
        }
    }
}

fn handle_window_change(
    windows: Res<Windows>,
    mut window_evr: EventReader<WindowResized>,
    mut q_fgtext: Query<&mut Text, With<FgText>>,
    mut q_bgtext: Query<&mut Text, (With<BgText>, Without<FgText>)>,
    mut q_nexttext: Query<&mut Text, (With<NextText>, Without<FgText>, Without<BgText>)>,
) {
    for _ in window_evr.iter() {
        let window = windows.get_primary().unwrap();

        let mut fgtext = q_fgtext.get_single_mut().unwrap();
        let mut bgtext = q_bgtext.get_single_mut().unwrap();
        let mut nexttext = q_nexttext.get_single_mut().unwrap();

        fgtext.sections[0].style.font_size = window.width() * FONT_SIZE_MULTIPLIER;
        bgtext.sections[0].style.font_size = window.width() * FONT_SIZE_MULTIPLIER;
        nexttext.sections[0].style.font_size = window.width() * FONT_SIZE_MULTIPLIER;
    }
}

pub struct ProgramPlugin;
impl Plugin for ProgramPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentKey(None))
            .insert_resource(NextWord("Placeholder".to_string()))
            .add_startup_system(setup_camera)
            .add_startup_system(setup_bg_text)
            .add_startup_system(setup_fg_text)
            .add_startup_system(setup_next_text)
            .add_system(handle_char)
            .add_system(handle_window_change);
    }
}
