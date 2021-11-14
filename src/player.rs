use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system()).add_startup_stage(
            "game_setup_place",
            SystemStage::single(player_spawn.system()),
        );
    }
}

const PLAYER1_SPRITE: &str = "penguin.png";
const PLAYER2_SPRITE: &str = "lion.png";

struct Materials {
    player1: Handle<ColorMaterial>,
    #[allow(unused)]
    player2: Handle<ColorMaterial>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Materials {
        player1: materials.add(asset_server.load(PLAYER1_SPRITE).into()),
        player2: materials.add(asset_server.load(PLAYER2_SPRITE).into()),
    });
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.player1.clone(),
        ..Default::default()
    });
}
