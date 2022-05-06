use bevy::prelude::*;
use wfc_solver::{
    description::{TileData, TileRotation},
    utils::Point,
};

use crate::{ui::LoadedSolution, wfc_asset::WfcProblemResource};

#[derive(Debug, Component)]
pub struct WorldTag;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(load_solution);
    }
}

fn load_solution(
    mut commands: Commands,
    wfc: Option<Res<WfcProblemResource>>,
    old_world: Query<Entity, With<WorldTag>>,
    query: Query<(Entity, &LoadedSolution)>,
) {
    let wfc = match wfc {
        Some(wfc) => wfc,
        None => return,
    };

    if let Ok((entity, LoadedSolution(solution))) = query.get_single() {
        if let Ok(entity) = old_world.get_single() {
            println!("Despawning old world");
            commands.entity(entity).despawn_recursive();
        }
        println!("Loading solution into world");
        commands
            .entity(entity)
            .remove::<LoadedSolution>()
            .insert(WorldTag)
            .insert_bundle(TransformBundle::default())
            .with_children(|cb| {
                let dimensions = solution.grid.dimensions();
                println!("Spawning children");
                for x in 0..dimensions.width() {
                    for y in 0..dimensions.height() {
                        for z in 0..dimensions.depth() {
                            let tile = solution
                                .grid
                                .get(Point::new(x, y, z))
                                .unwrap()
                                .iter_ones()
                                .next()
                                .unwrap();

                            let data = solution.compiled.data(tile);

                            if let Some(scene) = get_scene_handle(data, &wfc) {
                                let mut transform = Transform::from_translation(Vec3::new(
                                    x as f32 * 1.0,
                                    y as f32 * 1.0,
                                    z as f32 * 1.0,
                                ));
                                println!("Data: {:?} - {:?}", data, transform);

                                if data.flipped {
                                    transform = transform.with_scale(Vec3::new(-1.0, 1.0, 1.0));
                                }

                                let rotation = match data.rotation {
                                    TileRotation::R0 => 0f32,
                                    TileRotation::R90 => 90f32,
                                    TileRotation::R180 => 180f32,
                                    TileRotation::R270 => 270f32,
                                }
                                .to_radians();

                                transform.rotate(Quat::from_rotation_y(rotation));

                                cb.spawn_bundle(TransformBundle {
                                    local: transform,
                                    ..Default::default()
                                })
                                .with_children(|cb| {
                                    cb.spawn_scene(scene);
                                });
                            }
                        }
                    }
                }
            });
    }
}

fn get_scene_handle(data: &TileData, wfc: &WfcProblemResource) -> Option<Handle<Scene>> {
    let tile = &wfc.description.tiles[data.tile_index];
    wfc.mapper[&tile.id]
        .as_ref()
        .map(|(_, handle)| handle.clone())
}
