use bevy::prelude::*;

mod place;

use place::CharacterPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(CharacterPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
