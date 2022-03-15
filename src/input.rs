use bevy::{app::AppExit, prelude::*};

fn handle_quit(input: Res<Input<KeyCode>>, mut quit_event: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        quit_event.send(AppExit);
    }
}

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_quit);
    }
}
