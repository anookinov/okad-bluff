use bevy::prelude::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

const DUKE_SPRITE: &str = "duke.png";
const ASSASSIN_SPRITE: &str = "assassin.png";
const CAPTAIN_SPRITE: &str = "captain.png";
const AMBASSADOR_SPRITE: &str = "ambassador.png";
const CONTESSA_SPRITE: &str = "contessa.png";

#[allow(unused)]
struct Materials {
    duke: Handle<ColorMaterial>,
    assassin: Handle<ColorMaterial>,
    captain: Handle<ColorMaterial>,
    ambassador: Handle<ColorMaterial>,
    contessa: Handle<ColorMaterial>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Materials {
        duke: materials.add(asset_server.load(DUKE_SPRITE).into()),
        assassin: materials.add(asset_server.load(ASSASSIN_SPRITE).into()),
        captain: materials.add(asset_server.load(CAPTAIN_SPRITE).into()),
        ambassador: materials.add(asset_server.load(AMBASSADOR_SPRITE).into()),
        contessa: materials.add(asset_server.load(CONTESSA_SPRITE).into()),
    });
}
