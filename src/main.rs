use bevy::prelude::*;

mod board;
mod player;

use board::BoardPlugin;
use player::PlayerPlugin;

#[allow(unused)]
const SCALE: f32 = 0.5;

#[allow(unused)]
struct WinSize {
    w: f32,
    h: f32,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Okad Bluff!!".to_string(),
            width: 1280.0,
            height: 720.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(BoardPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    let window = windows
        .get_primary_mut()
        .expect("Cannot get a mutable primary window");

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });

    window.set_position(IVec2::new(0, 0));
}
