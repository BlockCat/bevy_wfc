use bitvec::prelude::BitVec;

use crate::{
    solver::ProblemSolver,
    utils::{Dimensions, FieldGrid, Point},
    Solution,
};

pub enum TileRotation {
    R0,
    R90,
    R180,
    R270,
}
pub struct Tile<D> {
    pub id: D,
    pub up: Vec<VerticalConnection>,
    pub down: Vec<VerticalConnection>,
    pub left: Vec<HorizontalConnection>,
    pub right: Vec<HorizontalConnection>,
    pub forward: Vec<HorizontalConnection>,
    pub backward: Vec<HorizontalConnection>,

    pub can_rotate: bool,
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

pub struct VerticalConnection {
    pub rotation: bool,
    pub connection: String,
}

pub struct HorizontalConnection {
    pub symmetry: bool,
    pub connection: String,
}

pub struct CompiledDescription {
    initial_grid: FieldGrid,
    dimensions: Dimensions,

    transformation: Vec<(usize, TileRotation, bool)>, // id -> (tile_id, rotation, flipped) from problemdescription + rotation
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

pub struct ProblemDescription<D> {
    pub connections: Vec<String>,
    pub dimensions: Dimensions,

    pub tiles: Vec<Tile<D>>,
    // pub fixed: Vec<FixedTile<D>>,
}

impl<D> ProblemDescription<D> {
    pub fn compile(&self) -> CompiledDescription {
        let count = self.tiles.len() * 4 * 2;
        let mut transformation = Vec::with_capacity(count);
        let mut up = Vec::new();
        let mut down = Vec::new();
        let mut forward = Vec::new();
        let mut backward = Vec::new();
        let mut left = Vec::new();
        let mut right = Vec::new();

        let mut initial_grid = FieldGrid::new(self.dimensions, BitVec::repeat(true, count));

        for (index, tile) in self.tiles.iter().enumerate() {
            let empty = BitVec::repeat(false, count);
            transformation.extend([
                (index, TileRotation::R0, false),
                (index, TileRotation::R90, false),
                (index, TileRotation::R180, false),
                (index, TileRotation::R270, false),
                (index, TileRotation::R0, true),
                (index, TileRotation::R90, true),
                (index, TileRotation::R180, true),
                (index, TileRotation::R270, true),
            ]);
            up.extend(vec![empty.clone(); 8]);
            down.extend(vec![empty.clone(); 8]);
            forward.extend(vec![empty.clone(); 8]);
            backward.extend(vec![empty.clone(); 8]);
            left.extend(vec![empty.clone(); 8]);
            right.extend(vec![empty.clone(); 8]);
        }

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
