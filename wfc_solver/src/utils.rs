use bitvec::{macros::internal::funty::Integral, prelude::BitVec};
use rand::distributions::uniform::{SampleUniform, UniformSampler};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dimensions {
    width: usize,
    height: usize,
    depth: usize,
}

impl Dimensions {
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn depth(&self) -> usize {
        self.depth
    }
    pub fn len(&self) -> usize {
        self.width * self.height * self.depth
    }
}

impl UniformSampler for Dimensions {
    type X = Point;

    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
    {
        unreachable!()
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
    {
        unreachable!()
    }

    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        let x = rng.gen_range(0..self.width());
        let y = rng.gen_range(0..self.height());
        let z = rng.gen_range(0..self.depth());

        Point::new(x, y, z)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    x: usize,
    y: usize,
    z: usize,
}

macro_rules! directional {
    (add $name:ident, $d:ident, $dim:ident, $x:expr) => {
        pub fn $name(self, dimensions: Dimensions) -> Option<Point> {
            if self.$d + 1 >= dimensions.$dim {
                None
            } else {
                Some(Point::new(self.x + $x.0, self.y + $x.1, self.z + $x.2))
            }
        }
    };

    (rem $name:ident, $d:ident, $x:expr) => {
        pub fn $name(self, _: Dimensions) -> Option<Point> {
            if self.$d == 0 {
                None
            } else {
                Some(Point::new(self.x - $x.0, self.y - $x.1, self.z - $x.2))
            }
        }
    };
}

impl Point {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Point { x, y, z }
    }
    pub fn x(&self) -> usize {
        self.x
    }
    pub fn y(&self) -> usize {
        self.y
    }
    pub fn z(&self) -> usize {
        self.z
    }
    pub fn id(&self, dimensions: Dimensions) -> usize {
        self.x + self.y * dimensions.width * dimensions.depth + self.z * dimensions.width
    }

    directional!(rem left, x,  (1, 0, 0));
    directional!(rem down, y, (0, 1, 0));
    directional!(rem backward, z, (0, 0, 1));
    directional!(add right, x, width, (1, 0, 0));
    directional!(add up, x, width, (0, 1, 0));
    directional!(add forward, x, width, (0, 0, 1));
}

#[derive(Debug, Clone)]
pub struct FieldGrid {
    dimensions: Dimensions,
    tiles: Vec<BitVec>,
}

impl FieldGrid {
    pub fn new(dimensions: Dimensions, initial: BitVec) -> Self {
        Self {
            tiles: vec![initial; dimensions.len()],
            dimensions,
        }
    }

    pub fn set(&mut self, point: Point, vec: BitVec) {
        self.tiles[point.id(self.dimensions)] = vec;
    }

    pub fn get(&self, point: Point) -> Option<&BitVec> {
        self.tiles.get(point.id(self.dimensions))
    }

    pub fn get_mut(&mut self, point: Point) -> Option<&mut BitVec> {
        self.tiles.get_mut(point.id(self.dimensions))
    }

    pub fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    pub fn tiles(&self) -> &Vec<BitVec> {
        &self.tiles
    }

    pub fn is_satisfiable(&self) -> bool {
        self.tiles.iter().all(|x| x.count_ones() >= 1)
    }
    pub fn is_complete(&self) -> bool {
        self.tiles.iter().all(|x| x.count_ones() == 1)
    }

    pub fn should_update(&self, point: Point, new_domain: &BitVec) -> bool {
        !self
            .get(point)
            .map(|bv| new_domain.contains(bv)) // new_domain >= bv
            .unwrap_or_default()
    }

    /**
     * Returns true if updated
     */
    pub fn update(&mut self, point: Point, new_domain: &BitVec) -> Option<&BitVec> {
        if self.should_update(point, new_domain) {
            let grid = self.get_mut(point).unwrap();
            *grid &= new_domain;
            return Some(grid);
        } else {
            return None;
        }
    }
}
