use std::{collections::VecDeque, ops::Range};

use bitvec::prelude::BitVec;
use rand::{
    distributions::uniform::UniformSampler,
    prelude::{IteratorRandom, SliceRandom},
    SeedableRng,
};

use crate::{
    description::{self, CompiledDescription},
    error::ProblemError,
    utils::{Dimensions, FieldGrid, Point},
};

use super::ProblemSolver;

#[derive(Debug, Default)]
pub struct NaiveSolver {}

impl ProblemSolver for NaiveSolver {
    fn solve(&mut self, description: &CompiledDescription) -> Result<FieldGrid, ProblemError> {
        if description.dimensions() != description.initial_grid().dimensions() {
            return Err(ProblemError::Dimensions);
        }

        let mut grid = FieldGrid::new(description.dimensions(), description.all_domain());

        update_initial_domain(&mut grid, description.initial_grid())?;
        update_initial_sides(&mut grid, description)?;

        if !grid.is_satisfiable() {
            return Err(ProblemError::Unsatisfiable);
        }

        let mut rng = rand::rngs::SmallRng::seed_from_u64(0);

        let initial_point = description.dimensions().sample(&mut rng);

        start_processs(&mut grid, initial_point, &mut rng, description)?;

        Ok(grid)
    }
}

fn update_initial_domain(
    grid: &mut FieldGrid,
    initial_grid: &FieldGrid,
) -> Result<(), ProblemError> {
    todo!("Update and propage tiles");
}

macro_rules! rekt {
    ($description:ident, $($dir:ident),*) => {{
        let mut vec = BitVec::repeat(false, $description.len());
        $description.all_domain()
        .iter_ones()
        $(
            .filter(|x| $description.$dir(*x)[0])
        )*
        .for_each(|x| vec.set(x, true));
        vec
    }};

    (range $grid:ident, $x:expr, $y:expr, $z:expr, $description:ident, $($dir:ident),*) => {{
        let v = rekt!($description, $($dir),*);
        let dx = $x.clone();
        for x in dx {
            for y in $y.clone() {
                for z in $z.clone() {
                    let p = Point::new(x, y, z);
                    propagate_point($grid, p, v.clone(), $description);
                }
            }
        }
    }};
}
fn update_initial_sides(
    grid: &mut FieldGrid,
    description: &CompiledDescription,
) -> Result<(), ProblemError> {
    let width = 0..grid.dimensions().width();
    let height = 0..grid.dimensions().height();
    let depth = 0..grid.dimensions().depth();

    let min_x = 0..1;
    // let min_y = 0..1;
    let min_z = 0..1;
    let max_x = grid.dimensions().width() - 1..grid.dimensions().width();
    let max_y = grid.dimensions().height() - 1..grid.dimensions().height();
    let max_z = grid.dimensions().depth() - 1..grid.dimensions().depth();

    // rekt!(range grid, width, min_y, depth, description, down);
    rekt!(range grid, width, max_y, depth, description, up);
    rekt!(range grid, min_x, height, depth, description, left);
    rekt!(range grid, max_x, height, depth, description, right);
    rekt!(range grid, width, height, max_z, description, forward);
    rekt!(range grid, width, height, min_z, description, backward);

    Ok(())
}

fn start_processs<R: rand::Rng>(
    grid: &mut FieldGrid,
    initial_point: Point,
    rng: &mut R,
    description: &CompiledDescription,
) -> Result<(), ProblemError> {
    let mut queue = VecDeque::new();

    queue.push_front(initial_point);

    while let Some(point) = queue.pop_front() {
        let point_vec = grid.get(point).unwrap();
        let fixed_index = point_vec
            .iter_ones()
            .choose(rng)
            .ok_or(ProblemError::Unsatisfiable)?;

        let mut vec = BitVec::repeat(false, point_vec.len());
        vec.set(fixed_index, true);

        propagate_point(grid, point, vec, description);
    }

    unimplemented!()
}

macro_rules! handle_direction {
    ($point:ident, $dimensions:ident, $domain:ident, $description:ident, $grid:ident, $queue:ident, $($direction:ident),+ ) => {{
        $(
            handle_direction!(dir $point, $dimensions, $domain, $description, $grid, $queue, $direction);
        )+
    }};
    (dir $point:ident, $dimensions:ident, $domain:ident, $description:ident, $grid:ident, $queue:ident, $direction:ident) => {{
        if let Some(neighbour_point) = $point.$direction($dimensions) {
            let potential_domain = potential_domain(&$domain, |i| $description.$direction(i).clone());
            if $grid.should_update(neighbour_point, &potential_domain) {
                $queue.push_back((neighbour_point, potential_domain));
            }
        }
    }};
}
fn propagate_point(
    grid: &mut FieldGrid,
    point: Point,
    vec: BitVec,
    description: &CompiledDescription,
) {
    let mut queue = VecDeque::new();
    queue.push_back((point, vec));

    while let Some((point, new_domain)) = queue.pop_front() {
        if let Some(updated) = grid.update(point, &new_domain) {
            let domain = updated.clone();
            let dimensions = grid.dimensions();

            handle_direction!(
                point,
                dimensions,
                domain,
                description,
                grid,
                queue,
                backward,
                forward,
                left,
                right,
                up,
                down
            );
        }
    }
}

fn potential_domain<F>(domain: &BitVec, factory: F) -> BitVec
where
    F: Fn(usize) -> BitVec,
{
    domain
        .iter_ones()
        .map(|i| factory(i))
        .fold(domain.clone(), |acc, x| acc & x)
}
