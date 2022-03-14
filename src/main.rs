mod words;
mod program;

use bevy::prelude::*;
use program::ProgramPlugin;
use crate::words::{Words, WordPlugin};

fn main() {
    let words = Words::new();

    App::new()
    	.insert_resource(WindowDescriptor {
    		title: "Zen Typer".to_string(),
    		mode: bevy::window::WindowMode::Fullscreen,
    		vsync: true,
    		resizable: false,
    		..Default::default()
    	})
		.add_plugin(ProgramPlugin)
		.add_plugin(WordPlugin)
    	.add_plugins(DefaultPlugins)
    	.run();
}
