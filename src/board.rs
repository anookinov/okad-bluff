use bevy::prelude::*;

use crate::WinSize;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_startup_stage("game_setup_cards", SystemStage::single(card_spawn.system()));
    }
}

const DUKE_SPRITE: &str = "duke.png";
const ASSASSIN_SPRITE: &str = "assassin.png";
const CAPTAIN_SPRITE: &str = "captain.png";
const AMBASSADOR_SPRITE: &str = "ambassador.png";
const CONTESSA_SPRITE: &str = "contessa.png";
const COIN_SPRITE: &str = "coin.png";
const SPRITE_SIZE: u32 = 250;
const DUKE_TOTAL: u8 = 3;
const ASSASSIN_TOTAL: u8 = 3;
const CAPTAIN_TOTAL: u8 = 3;
const AMBASSADOR_TOTAL: u8 = 3;
const CONTESSA_TOTAL: u8 = 3;
const COIN_TOTAL: u8 = 50;
const SPRITE_GAP: u32 = 50;

#[allow(unused)]
struct Materials {
    duke: Handle<ColorMaterial>,
    assassin: Handle<ColorMaterial>,
    captain: Handle<ColorMaterial>,
    ambassador: Handle<ColorMaterial>,
    contessa: Handle<ColorMaterial>,
    coin: Handle<ColorMaterial>,
}

struct Duke;
struct Assassin;

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
        coin: materials.add(asset_server.load(COIN_SPRITE).into()),
    });
}

fn card_spawn(mut commands: Commands, materials: Res<Materials>, win_size: Res<WinSize>) {
    let top = win_size.h / 2.0;
    let left = -win_size.w / 2.0;
    let mut count = 0;

    for _ in 0..DUKE_TOTAL {
        let gap = SPRITE_GAP as f32;
        let x_offset = SPRITE_SIZE as f32;
        let y_offset = SPRITE_SIZE as f32;
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.duke.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        left + x_offset / 2.0 + count as f32 * gap,
                        top - y_offset / 2.0,
                        0.0,
                    ),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Duke);
        count += 1;
    }

    for _ in 0..ASSASSIN_TOTAL {
        let gap = SPRITE_GAP as f32;
        let x_offset = SPRITE_SIZE as f32;
        let y_offset = SPRITE_SIZE as f32;
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.assassin.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        left + x_offset / 2.0 + count as f32 * gap,
                        top - y_offset / 2.0,
                        0.0,
                    ),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Assassin);
        count += 1;
    }
}
