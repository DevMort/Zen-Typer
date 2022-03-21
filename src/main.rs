mod input;
mod program;
mod sfx;
mod words;

use crate::input::InputPlugin;
use crate::words::Words;
use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use program::ProgramPlugin;
use sfx::SFXPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Zen Typer".to_string(),
            width: 840.0,
            height: 600.0,
            ..Default::default()
        })
        .insert_resource(Words::new())
        .add_plugin(ProgramPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(SFXPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .run();
}
