use bevy::prelude::*;

fn setup_camera(mut commands: Commands) {
	commands.spawn_bundle(UiCameraBundle::default());
}

pub struct ProgramPlugin;
impl Plugin for ProgramPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(setup_camera);
	}
}
