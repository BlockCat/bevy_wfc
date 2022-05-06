use super::ProblemSolver;
use crate::{
    description::CompiledDescription,
    error::ProblemError,
    utils::{FieldGrid, Point},
};
use bitvec::prelude::BitVec;
use rand::{prelude::IteratorRandom, Rng};
use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct NaiveSolver {}

impl ProblemSolver for NaiveSolver {
    fn solve<R: Rng>(
        &mut self,
        rng: &mut R,
        description: &CompiledDescription,
    ) -> Result<FieldGrid, ProblemError> {
        // if description.dimensions() != description.initial_grid().dimensions() {
        //     return Err(ProblemError::Dimensions);
        // }

        let mut grid = FieldGrid::new(description.dimensions(), description.all_domain());

        // update_initial_domain(&mut grid, description.initial_grid())?;
        update_initial_sides(&mut grid, description)?;
        print_dimensions(&grid);

        if !grid.is_satisfiable() {
            // panic!("Error: It went wrong with initial sides");
            return Err(ProblemError::Unsatisfiable);
        }

        start_processs(&mut grid, rng, description)?;

        Ok(grid)
    }
}

fn print_dimensions(grid: &FieldGrid) {
    println!("Dimensions: {:?}", grid.dimensions());
    for y in 0..grid.dimensions().height() {
        for z in 0..grid.dimensions().depth() {
            for x in 0..grid.dimensions().width() {
                println!(
                    "({}, {}, {}) - {}",
                    x,
                    y,
                    z,
                    grid.get(Point::new(x, y, z)).unwrap()
                );
            }
        }
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
                    propagate_point($grid, p, v.clone(), $description)?;
                }
            }
        }
    }};
}
pub fn update_initial_sides(
    grid: &mut FieldGrid,
    description: &CompiledDescription,
) -> Result<(), ProblemError> {
    let width = dbg!(0..grid.dimensions().width());
    let height = dbg!(0..grid.dimensions().height());
    let depth = dbg!(0..grid.dimensions().depth());

    let min_x = 0..1;
    // let min_y = 0..1;
    let min_z = 0..1;
    let max_x = (grid.dimensions().width() - 1)..grid.dimensions().width();
    let max_y = (grid.dimensions().height() - 1)..grid.dimensions().height();
    let max_z = (grid.dimensions().depth() - 1)..grid.dimensions().depth();

    // // rekt!(range grid, width, min_y, depth, description, down);

    rekt!(range grid, width, max_y, depth, description, up);
    rekt!(range grid, min_x, height, depth, description, left);
    rekt!(range grid, max_x, height, depth, description, right);
    rekt!(range grid, width, height, max_z, description, forward);
    rekt!(range grid, width, height, min_z, description, backward);

    Ok(())
}

fn start_processs<R: rand::Rng>(
    grid: &mut FieldGrid,
    rng: &mut R,
    description: &CompiledDescription,
) -> Result<(), ProblemError> {
    let mut points = Vec::with_capacity(grid.dimensions().len());
    for x in 0..grid.dimensions().width() {
        for y in 0..grid.dimensions().height() {
            for z in 0..grid.dimensions().depth() {
                points.push(Point::new(x, y, z));
            }
        }
    }

    while !grid.is_complete() {
        points = points
            .into_iter()
            .filter(|x| grid.get(*x).unwrap().count_ones() > 1)
            .collect::<Vec<_>>();

        if let Some(point) = points
            .iter()
            .min_by_key(|x| grid.get(**x).unwrap().count_ones())
        {
            let point_vec = grid.get(*point).unwrap();

            let fixed_index = point_vec
                .iter_ones()
                .choose(rng)
                .ok_or(ProblemError::Unsatisfiable)?;

            let mut vec = BitVec::repeat(false, point_vec.len());
            vec.set(fixed_index, true);

            propagate_point(grid, *point, vec, description)?;
        }
    }

    Ok(())
}

macro_rules! handle_direction {
    ($point:ident, $dimensions:ident, $domain:ident, $description:ident, $grid:ident, $queue:ident, $($direction:ident),+ ) => {{
        $(
            handle_direction!(dir $point, $dimensions, $domain, $description, $grid, $queue, $direction);
        )+
    }};
    (dir $point:ident, $dimensions:ident, $domain:ident, $description:ident, $grid:ident, $queue:ident, $direction:ident) => {{
        if let Some(neighbour_point) = $point.$direction($dimensions) {
            let potential_domain = (potential_domain((&$domain), |i| $description.$direction(i).clone()));
            if $grid.should_update(neighbour_point, &potential_domain) {
                $queue.push_back((neighbour_point, potential_domain));
            }
        }
    }};
}
pub fn propagate_point(
    grid: &mut FieldGrid,
    point: Point,
    vec: BitVec,
    description: &CompiledDescription,
) -> Result<(), ProblemError> {
    let mut queue = VecDeque::new();
    queue.push_back((point, vec));

    while let Some((point, new_domain)) = queue.pop_front() {
        if let Some(updated) = grid.update(point, &new_domain) {
            if updated.count_ones() == 0 {
                return Err(ProblemError::Unsatisfiable);
            }
            let domain = updated.clone();
            let dimensions = grid.dimensions();

            handle_direction!(
                point,
                dimensions,
                domain,
                description,
                grid,
                queue,
                up,
                down,
                backward,
                forward,
                left,
                right
            );
        }
    }

    // println!("Propagated: {:?} -- {}", point, propagated);

    Ok(())
}

fn potential_domain<F>(domain: &BitVec, factory: F) -> BitVec
where
    F: Fn(usize) -> BitVec,
{
    domain
        .iter_ones()
        .map(|i| factory(i))
        .fold(BitVec::repeat(false, domain.len()), |acc, x| acc | x)
}
