use bevy::prelude::*;

pub struct PlacePlugin;

impl Plugin for PlacePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system()).add_startup_stage(
            "game_setup_place",
            SystemStage::single(place_spawn.system()),
        );
    }
}

const HOTEL_SPRITE: &str = "hotel.png";
const HOSPITAL_SPRITE: &str = "hospital.png";

struct Materials {
    hotel: Handle<ColorMaterial>,
    #[allow(unused)]
    hospital: Handle<ColorMaterial>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Materials {
        hotel: materials.add(asset_server.load(HOTEL_SPRITE).into()),
        hospital: materials.add(asset_server.load(HOSPITAL_SPRITE).into()),
    });
}

fn place_spawn(mut commands: Commands, materials: Res<Materials>) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.hotel.clone(),
        ..Default::default()
    });
}
