use bevy::prelude::*;
use bevy_kira_audio::Audio;

pub struct ClickEvent;

fn click_sfx(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut ev_click: EventReader<ClickEvent>,
) {
    for _ in ev_click.iter() {
        audio.play(asset_server.load("click.wav"));
    }
}

pub struct SFXPlugin;
impl Plugin for SFXPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClickEvent>().add_system(click_sfx);
    }
}
