use bevy::prelude::*;
mod libs;
mod plugins;
use plugins::player::PlayerPlugin;
use plugins::camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}