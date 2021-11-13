use bevy::prelude::*;

mod place;

use place::PlacePlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlacePlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
