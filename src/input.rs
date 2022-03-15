use bevy::{app::AppExit, prelude::*};

use crate::program::{CurrentKey, FgText};

fn handle_quit(input: Res<Input<KeyCode>>, mut quit_event: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        quit_event.send(AppExit);
    }
}

fn handle_keypress(
    input: Res<Input<KeyCode>>,
    mut current_key: ResMut<CurrentKey>,
    mut q_fgkey: Query<&mut Text, With<FgText>>,
) {
    // backspace
    if input.just_pressed(KeyCode::Back) {
        let mut fgkey = q_fgkey.get_single_mut().unwrap();
        fgkey.sections[0].value.pop();
    }

    // lowercases
    if input.just_pressed(KeyCode::A) {
        current_key.0 = Some('a');
    } else if input.just_pressed(KeyCode::B) {
        current_key.0 = Some('b');
    } else if input.just_pressed(KeyCode::C) {
        current_key.0 = Some('c');
    } else if input.just_pressed(KeyCode::D) {
        current_key.0 = Some('d');
    } else if input.just_pressed(KeyCode::E) {
        current_key.0 = Some('e');
    } else if input.just_pressed(KeyCode::F) {
        current_key.0 = Some('f');
    } else if input.just_pressed(KeyCode::G) {
        current_key.0 = Some('g');
    } else if input.just_pressed(KeyCode::H) {
        current_key.0 = Some('h');
    } else if input.just_pressed(KeyCode::I) {
        current_key.0 = Some('i');
    } else if input.just_pressed(KeyCode::J) {
        current_key.0 = Some('j');
    } else if input.just_pressed(KeyCode::K) {
        current_key.0 = Some('k');
    } else if input.just_pressed(KeyCode::L) {
        current_key.0 = Some('l');
    } else if input.just_pressed(KeyCode::M) {
        current_key.0 = Some('m');
    } else if input.just_pressed(KeyCode::N) {
        current_key.0 = Some('n');
    } else if input.just_pressed(KeyCode::O) {
        current_key.0 = Some('o');
    } else if input.just_pressed(KeyCode::P) {
        current_key.0 = Some('p');
    } else if input.just_pressed(KeyCode::Q) {
        current_key.0 = Some('q');
    } else if input.just_pressed(KeyCode::R) {
        current_key.0 = Some('r');
    } else if input.just_pressed(KeyCode::S) {
        current_key.0 = Some('s');
    } else if input.just_pressed(KeyCode::T) {
        current_key.0 = Some('t');
    } else if input.just_pressed(KeyCode::U) {
        current_key.0 = Some('u');
    } else if input.just_pressed(KeyCode::V) {
        current_key.0 = Some('v');
    } else if input.just_pressed(KeyCode::W) {
        current_key.0 = Some('w');
    } else if input.just_pressed(KeyCode::X) {
        current_key.0 = Some('x');
    } else if input.just_pressed(KeyCode::Y) {
        current_key.0 = Some('y');
    } else if input.just_pressed(KeyCode::Z) {
        current_key.0 = Some('z');
    }

    // uppercases
    if input.pressed(KeyCode::LShift) || input.pressed(KeyCode::RShift) {
        if input.just_pressed(KeyCode::A) {
            current_key.0 = Some('A');
        } else if input.just_pressed(KeyCode::B) {
            current_key.0 = Some('B');
        } else if input.just_pressed(KeyCode::C) {
            current_key.0 = Some('C');
        } else if input.just_pressed(KeyCode::D) {
            current_key.0 = Some('D');
        } else if input.just_pressed(KeyCode::E) {
            current_key.0 = Some('E');
        } else if input.just_pressed(KeyCode::F) {
            current_key.0 = Some('F');
        } else if input.just_pressed(KeyCode::G) {
            current_key.0 = Some('G');
        } else if input.just_pressed(KeyCode::H) {
            current_key.0 = Some('H');
        } else if input.just_pressed(KeyCode::I) {
            current_key.0 = Some('I');
        } else if input.just_pressed(KeyCode::J) {
            current_key.0 = Some('J');
        } else if input.just_pressed(KeyCode::K) {
            current_key.0 = Some('K');
        } else if input.just_pressed(KeyCode::L) {
            current_key.0 = Some('L');
        } else if input.just_pressed(KeyCode::M) {
            current_key.0 = Some('M');
        } else if input.just_pressed(KeyCode::N) {
            current_key.0 = Some('N');
        } else if input.just_pressed(KeyCode::O) {
            current_key.0 = Some('O');
        } else if input.just_pressed(KeyCode::P) {
            current_key.0 = Some('P');
        } else if input.just_pressed(KeyCode::Q) {
            current_key.0 = Some('Q');
        } else if input.just_pressed(KeyCode::R) {
            current_key.0 = Some('R');
        } else if input.just_pressed(KeyCode::S) {
            current_key.0 = Some('S');
        } else if input.just_pressed(KeyCode::T) {
            current_key.0 = Some('T');
        } else if input.just_pressed(KeyCode::U) {
            current_key.0 = Some('U');
        } else if input.just_pressed(KeyCode::V) {
            current_key.0 = Some('V');
        } else if input.just_pressed(KeyCode::W) {
            current_key.0 = Some('W');
        } else if input.just_pressed(KeyCode::X) {
            current_key.0 = Some('X');
        } else if input.just_pressed(KeyCode::Y) {
            current_key.0 = Some('Y');
        } else if input.just_pressed(KeyCode::Z) {
            current_key.0 = Some('Z');
        }
    }
}

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_quit).add_system(handle_keypress);
    }
}
