use bevy::prelude::*;

fn main() {
	App::new()
		.insert_resource(WindowDescriptor {
			title: "Zen Typer".to_string(),
			mode: bevy::window::WindowMode::Fullscreen,
			vsync: true,
			resizable: false,
			..Default::default()
		})
		.add_plugins(DefaultPlugins)
		.run();
}
