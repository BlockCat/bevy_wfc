use std::collections::HashMap;

use crate::utils::{Dimensions, FieldGrid, Point};
use bitvec::prelude::BitVec;
use serde::Deserialize;

pub enum TileRotation {
    R0,
    R90,
    R180,
    R270,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Tile<D> {
    pub id: D,
    pub up: Vec<VerticalConnection>,
    pub down: Vec<VerticalConnection>,
    pub left: Vec<HorizontalConnection>,
    pub right: Vec<HorizontalConnection>,
    pub forward: Vec<HorizontalConnection>,
    pub backward: Vec<HorizontalConnection>,

    pub can_rotate: bool,
    pub can_flip: bool,
}

pub struct FixedTile<D> {
    pub point: Point,
    pub rotation: TileRotation,
    pub tile: Tile<D>,
}

impl<D> FixedTile<D> {
    pub fn new(point: Point, rotation: TileRotation, tile: Tile<D>) -> Self {
        Self {
            point,
            rotation,
            tile,
        }
    }
}
#[derive(Debug, Default, Clone, Deserialize)]
pub struct VerticalConnection {
    // pub rotation: bool,
    pub connection: String,
}

impl VerticalConnection {
    pub fn is_connected(&self, other: &Self) -> bool {
        self.connection == other.connection
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct HorizontalConnection {
    pub symmetry: bool,
    pub flipped: bool,
    pub connection: String,
}

pub struct TileData {
    tile_index: usize,
    rotation: TileRotation,
    flipped: bool,
}

pub struct CompiledDescription {
    initial_grid: FieldGrid,
    dimensions: Dimensions,

    transformation: Vec<TileData>, // id -> (tile_id, rotation, flipped) from problemdescription + rotation
    up: Vec<BitVec>,
    down: Vec<BitVec>,
    left: Vec<BitVec>,
    right: Vec<BitVec>,
    forward: Vec<BitVec>,
    backward: Vec<BitVec>,
}

macro_rules! directional {
    ($x:ident) => {
        pub fn $x(&self, index: usize) -> &BitVec {
            &self.$x[index]
        }
    };
}

impl CompiledDescription {
    pub fn initial_grid(&self) -> &FieldGrid {
        &self.initial_grid
    }
    pub fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    pub fn all_domain(&self) -> BitVec {
        BitVec::repeat(true, self.transformation.len())
    }

    pub fn len(&self) -> usize {
        self.transformation.len()
    }

    directional!(up);
    directional!(down);
    directional!(backward);
    directional!(forward);
    directional!(left);
    directional!(right);
}

#[derive(Debug, Clone)]
pub struct ProblemDescription<D> {
    pub connections: Vec<String>,
    pub dimensions: Dimensions,

    pub tiles: Vec<Tile<D>>,
    // pub fixed: Vec<FixedTile<D>>,
}

macro_rules! tile_data {
    ($a:expr, $b:expr, $c:expr) => {
        TileData {
            tile_index: $a,
            rotation: $b,
            flipped: $c,
        }
    };
}

impl<D> ProblemDescription<D> {
    pub fn compile(&self) -> CompiledDescription {
        let count = self.tiles.len() * 4 * 2;
        let mut transformation = Vec::with_capacity(count);

        let mut initial_grid = FieldGrid::new(self.dimensions, BitVec::repeat(true, count));

        for (index, tile) in self.tiles.iter().enumerate() {
            let empty = BitVec::repeat(false, count);
            transformation.extend([
                tile_data!(index, TileRotation::R0, false),
                tile_data!(index, TileRotation::R90, false),
                tile_data!(index, TileRotation::R180, false),
                tile_data!(index, TileRotation::R270, false),
                tile_data!(index, TileRotation::R0, true),
                tile_data!(index, TileRotation::R90, true),
                tile_data!(index, TileRotation::R180, true),
                tile_data!(index, TileRotation::R270, true),
            ]);
        }

        let up = collect_up_connections(&self.tiles, &transformation);
        let down = collect_down_connections(&self.tiles, &transformation);

        CompiledDescription {
            dimensions: self.dimensions,
            transformation,
            initial_grid,
            up,
            down,
            left,
            right,
            forward,
            backward,
        }
    }
}

fn collect_up_connections<D>(tiles: &Vec<Tile<D>>, transformations: &Vec<TileData>) -> Vec<BitVec> {
    let empty = BitVec::repeat(false, transformations.len());
    transformations
        .into_iter()
        .enumerate()
        .map(|(index, td)| {
            let mut v = empty.clone();
            for i in collect_up_nodes(index, tiles, transformations) {
                v.set(i, true);
            }
            v
        })
        .collect()
}

fn collect_up_nodes<D>(
    tile: usize,
    tiles: &Vec<Tile<D>>,
    transformations: &Vec<TileData>,
) -> Vec<usize> {
    let up_connections = &tiles[transformations[tile].tile_index].up;

    transformations
        .into_iter()
        .enumerate()
        .filter(|(_, td)| {
            let tile = &tiles[td.tile_index].down;
            up_connections
                .into_iter()
                .any(|uc| tile.into_iter().any(|c| uc.is_connected(c)))
        })
        .map(|x| x.0)
        .collect()
}

fn collect_down_connections<D>(
    tiles: &Vec<Tile<D>>,
    transformations: &Vec<TileData>,
) -> Vec<BitVec> {
    let empty = BitVec::repeat(false, transformations.len());
    transformations
        .into_iter()
        .enumerate()
        .map(|(index, td)| {
            let mut v = empty.clone();
            for i in collect_down_nodes(index, tiles, transformations) {
                v.set(i, true);
            }
            v
        })
        .collect()
}

fn collect_down_nodes<D>(
    tile: usize,
    tiles: &Vec<Tile<D>>,
    transformations: &Vec<TileData>,
) -> Vec<usize> {
    let down_connections = &tiles[transformations[tile].tile_index].down;

    transformations
        .into_iter()
        .enumerate()
        .filter(|(_, td)| {
            let tile = &tiles[td.tile_index].up;
            down_connections
                .into_iter()
                .any(|uc| tile.into_iter().any(|c| uc.is_connected(c)))
        })
        .map(|x| x.0)
        .collect()
}
