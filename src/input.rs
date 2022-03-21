use bevy::{app::AppExit, prelude::*};

use crate::program::{CurrentKey, FgText};

fn handle_keypress(
    mut char_evr: EventReader<ReceivedCharacter>,
    mut current_key: ResMut<CurrentKey>,
    input: Res<Input<KeyCode>>,
    mut q_fgkey: Query<&mut Text, With<FgText>>,
    mut quit_event: EventWriter<AppExit>,
) {
    if input.just_pressed(KeyCode::Escape) {
        quit_event.send(AppExit);
    }

    if input.just_pressed(KeyCode::Back) {
        let mut fgkey = q_fgkey.get_single_mut().unwrap();
        fgkey.sections[0].value.pop();
    }

    for ev in char_evr.iter() {
        if input.is_changed()
            && (ev.char.is_ascii_alphabetic() || ev.char.is_ascii_graphic() || ev.char == ' ')
        {
            current_key.0 = Some(ev.char);
        }
    }
}

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_keypress);
    }
}
