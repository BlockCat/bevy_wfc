use bevy::{prelude::*, reflect::TypeUuid};
use bevy_asset_ron::RonAssetPlugin;
use serde::Deserialize;
use std::collections::HashMap;
use wfc_solver::{
    description::{ProblemDescription, Tile},
    utils::Dimensions,
};

#[derive(Debug, Default, Deserialize, TypeUuid)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268f8"]
pub struct TileAsset {
    pub scene: Option<TileMesh>,
    pub tile: Tile<String>,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct TileMesh {
    pub path: String,
    pub position: Vec3,
    pub scale: Vec3,
}

#[derive(Debug, Clone, Default, Deserialize, TypeUuid)]
#[uuid = "3178ccfa-d18a-4a9f-be8b-b3233f77510b"]
pub struct ProblemDescriptionAsset {
    dimensions: (usize, usize, usize),
    connections: Vec<String>,
    tiles: Vec<String>,
}

#[derive(Component)]
struct LoadingProblemTag;

#[derive(Component)]
struct LoadingTilesTag(ProblemDescriptionAsset, Vec<Handle<TileAsset>>);
pub struct WfcAssetPlugin;

impl Plugin for WfcAssetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(RonAssetPlugin::<TileAsset>::new(&["tile"]))
            .add_plugin(RonAssetPlugin::<ProblemDescriptionAsset>::new(&["desc"]))
            .add_startup_system(setup_description)
            .add_system(description_asset_loader)
            .add_system(tile_asset_loader.after(description_asset_loader));
    }
}

fn setup_description(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert(LoadingProblemTag)
        .insert(asset_server.load::<ProblemDescriptionAsset, &str>("description.desc"));
}

fn description_asset_loader(
    mut commands: Commands,
    res: Res<Assets<ProblemDescriptionAsset>>,
    query: Query<(Entity, &Handle<ProblemDescriptionAsset>), With<LoadingProblemTag>>,
    asset_server: Res<AssetServer>,
) {
    query.for_each(|(entity, handle)| {
        if let Some(description) = res.get(handle) {
            let handles = description
                .tiles
                .iter()
                .map(|x: &String| -> Handle<TileAsset> { asset_server.load(x) })
                .collect::<Vec<_>>();

            commands
                .entity(entity)
                .remove::<LoadingProblemTag>()
                .insert(LoadingTilesTag(description.clone(), handles));
        }
    });
}

fn tile_asset_loader(
    mut commands: Commands,
    res: Res<Assets<TileAsset>>,
    query: Query<(Entity, &LoadingTilesTag)>,
    asset_server: Res<AssetServer>,
) {
    query.for_each(|(entity, LoadingTilesTag(description, handles))| {
        let handles = handles.iter().map(|x| res.get(x)).collect::<Vec<_>>();
        if handles.iter().all(|x| x.is_some()) {
            let tiles = handles
                .iter()
                .map(|x| x.unwrap())
                .collect::<Vec<&TileAsset>>();

            let description = ProblemDescription {
                connections: description.connections.clone(),
                dimensions: Dimensions::new(
                    description.dimensions.0,
                    description.dimensions.1,
                    description.dimensions.2,
                ),
                tiles: tiles
                    .iter()
                    .map(|x| x.tile.clone())
                    .collect::<Vec<Tile<String>>>(),
            };

            let mapper = tiles
                .iter()
                .map(|x: &&TileAsset| {
                    (
                        x.tile.id.clone(),
                        x.scene
                            .clone()
                            .map(|scene: TileMesh| (scene.clone(), asset_server.load(&scene.path))),
                    )
                })
                .collect();

            let resource = WfcProblemResource {
                description,
                mapper,
            };
            commands.insert_resource(resource);

            commands.entity(entity).despawn_recursive();
        }
    });
}

#[derive(Debug)]
pub struct WfcProblemResource {
    pub description: ProblemDescription<String>,
    pub mapper: HashMap<String, Option<(TileMesh, Handle<Scene>)>>,
}
