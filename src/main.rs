mod words;

use words::Words;

// use bevy::prelude::*;

fn main() {
    let words = Words::new();
	println!("{}", words.get_new_word());

    // App::new()
    // 	.insert_resource(WindowDescriptor {
    // 		title: "Zen Typer".to_string(),
    // 		mode: bevy::window::WindowMode::Fullscreen,
    // 		vsync: true,
    // 		resizable: false,
    // 		..Default::default()
    // 	})
    // 	.add_plugins(DefaultPlugins)
    // 	.run();
}
