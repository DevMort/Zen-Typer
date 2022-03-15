mod input;
mod program;
mod words;

use crate::input::InputPlugin;
use crate::words::{WordPlugin, Words};
use bevy::prelude::*;
use program::ProgramPlugin;

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
        .add_plugin(InputPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
