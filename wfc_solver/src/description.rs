use crate::utils::{Dimensions, Point};
use bitvec::prelude::BitVec;
use serde::Deserialize;
use std::ops::Neg;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileRotation {
    R0,
    R90,
    R180,
    R270,
}

impl Neg for TileRotation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            TileRotation::R0 => Self::R180,
            TileRotation::R90 => Self::R270,
            TileRotation::R180 => Self::R0,
            TileRotation::R270 => Self::R90,
        }
    }
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

impl<D> Tile<D> {
    pub fn get_horizontal_connection(
        &self,
        want_direction: TileRotation,
        facing_direction: TileRotation,
    ) -> &Vec<HorizontalConnection> {
        match (want_direction, facing_direction) {
            (TileRotation::R0, TileRotation::R0) => &self.forward,
            (TileRotation::R0, TileRotation::R90) => &self.left,
            (TileRotation::R0, TileRotation::R180) => &self.backward,
            (TileRotation::R0, TileRotation::R270) => &self.right,

            (TileRotation::R90, TileRotation::R0) => &self.right,
            (TileRotation::R90, TileRotation::R90) => &self.forward,
            (TileRotation::R90, TileRotation::R180) => &self.left,
            (TileRotation::R90, TileRotation::R270) => &self.backward,

            (TileRotation::R180, TileRotation::R0) => &self.backward,
            (TileRotation::R180, TileRotation::R90) => &self.right,
            (TileRotation::R180, TileRotation::R180) => &self.forward,
            (TileRotation::R180, TileRotation::R270) => &self.left,

            (TileRotation::R270, TileRotation::R0) => &self.left,
            (TileRotation::R270, TileRotation::R90) => &self.backward,
            (TileRotation::R270, TileRotation::R180) => &self.right,
            (TileRotation::R270, TileRotation::R270) => &self.forward,
        }
    }
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

impl HorizontalConnection {
    pub fn is_connected(&self, other: &Self, self_flipped: bool, other_flipped: bool) -> bool {
        if self.connection != other.connection {
            return false;
        }

        if self.symmetry || other.symmetry {
            return true;
        }

        // result | flipped | self_flipped
        // false  | true    | true
        // true   | true    | false
        // false  | false   | false
        // true   | false   | true
        let self_flipped = self.flipped ^ self_flipped;
        let other_flipped = other.flipped ^ other_flipped;

        // return true;

        return self_flipped != other_flipped;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TileData {
    pub tile_index: usize,
    pub rotation: TileRotation,
    pub flipped: bool,
}

impl TileData {
    pub fn new(index: usize, rotation: TileRotation, flipped: bool) -> Self {
        TileData {
            tile_index: index,
            rotation,
            flipped,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledDescription {
    // initial_grid: FieldGrid,
    pub dimensions: Dimensions,

    pub transformation: Vec<TileData>, // id -> (tile_id, rotation, flipped) from problemdescription + rotation
    pub up: Vec<BitVec>,
    pub down: Vec<BitVec>,
    pub left: Vec<BitVec>,
    pub right: Vec<BitVec>,
    pub forward: Vec<BitVec>,
    pub backward: Vec<BitVec>,
}

macro_rules! directional {
    ($x:ident) => {
        pub fn $x(&self, index: usize) -> &BitVec {
            &self.$x[index]
        }
    };
}

impl CompiledDescription {
    // pub fn initial_grid(&self) -> &FieldGrid {
    //     &self.initial_grid
    // }
    pub fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    pub fn all_domain(&self) -> BitVec {
        BitVec::repeat(true, self.transformation.len())
    }

    pub fn len(&self) -> usize {
        self.transformation.len()
    }

    pub fn data(&self, index: usize) -> &TileData {
        &self.transformation[index]
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
        let count = self.tiles.len() * 4;
        let mut transformation = Vec::with_capacity(count);

        // let initial_grid = FieldGrid::new(self.dimensions, BitVec::repeat(true, count));

        for (index, _) in self.tiles.iter().enumerate() {
            transformation.extend([
                tile_data!(index, TileRotation::R0, false),
                tile_data!(index, TileRotation::R90, false),
                tile_data!(index, TileRotation::R180, false),
                tile_data!(index, TileRotation::R270, false),
                // tile_data!(index, TileRotation::R0, true),
                // tile_data!(index, TileRotation::R90, true),
                // tile_data!(index, TileRotation::R180, true),
                // tile_data!(index, TileRotation::R270, true),
            ]);
        }

        let up = collect_up_connections(&self.tiles, &transformation);
        let down = collect_down_connections(&self.tiles, &transformation);
        let left = collect_direction_connections(&self.tiles, &transformation, TileRotation::R270);
        let right = collect_direction_connections(&self.tiles, &transformation, TileRotation::R90);
        let forward = collect_direction_connections(&self.tiles, &transformation, TileRotation::R0);
        let backward =
            collect_direction_connections(&self.tiles, &transformation, TileRotation::R180);

        CompiledDescription {
            dimensions: self.dimensions,
            transformation,
            // initial_grid,
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
        .map(|(index, _)| {
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
        .map(|(index, _)| {
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

fn collect_direction_connections<D>(
    tiles: &Vec<Tile<D>>,
    transformations: &Vec<TileData>,
    tile_direction: TileRotation,
) -> Vec<BitVec> {
    let empty = BitVec::repeat(false, transformations.len());
    transformations
        .into_iter()
        .enumerate()
        .map(|(index, td)| {
            let mut v = empty.clone();
            for i in collect_direction_nodes(index, tiles, transformations, td, tile_direction) {
                v.set(i, true);
            }
            v
        })
        .collect()
}

fn collect_direction_nodes<D>(
    transform_index: usize,
    tiles: &Vec<Tile<D>>,
    transformations: &Vec<TileData>,
    tile_data: &TileData,
    tile_direction: TileRotation,
) -> Vec<usize> {
    let connections = &tiles[transformations[transform_index].tile_index];
    let connections = connections.get_horizontal_connection(tile_direction, tile_data.rotation);
    transformations
        .into_iter()
        .enumerate()
        .filter(|(_, td)| {
            let tile = &tiles[td.tile_index];
            let tile = tile.get_horizontal_connection(-tile_direction, td.rotation);
            connections.into_iter().any(|uc| {
                tile.into_iter()
                    .any(|c| uc.is_connected(c, tile_data.flipped, td.flipped))
            })
        })
        .map(|x| x.0)
        .collect()
}
