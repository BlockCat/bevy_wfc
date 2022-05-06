use description::{CompiledDescription, ProblemDescription};
use error::ProblemError;
use solver::{naive::NaiveSolver, ProblemSolver};
use utils::FieldGrid;

pub mod error;
pub mod solver;
pub mod utils;

pub mod description;

pub struct Solution<D> {
    pub grid: FieldGrid,
    pub description: ProblemDescription<D>,
    pub compiled: CompiledDescription,
}

pub fn solve<D, R: rand::Rng>(
    rng: &mut R,
    description: ProblemDescription<D>,
) -> Result<Solution<D>, ProblemError> {
    let compiled = description.compile();

    println!("Compiled");
    println!("Forward");
    for x in 0..compiled.len() {
        println!("{:?} -- {}", x, compiled.forward(x));
    }

    println!("Backward");
    for x in 0..compiled.len() {
        println!("{:?} -- {}", x, compiled.backward(x));
    }

    println!("Left");
    for x in 0..compiled.len() {
        println!("{:?} -- {}", x, compiled.left(x));
    }

    println!("Right");
    for x in 0..compiled.len() {
        println!("{:?} -- {}", x, compiled.right(x));
    }

    println!("Up");
    for x in 0..compiled.len() {
        println!("{:?} -- {}", x, compiled.up(x));
    }

    println!("Down");
    for x in 0..compiled.len() {
        println!("{:?} -- {}", x, compiled.down(x));
    }

    let mut solver = NaiveSolver::default();

    let grid = solver.solve(rng, &compiled)?;

    for t in grid.tiles() {
        println!("s: {}", t);
    }

    Ok(Solution {
        grid,
        description,
        compiled,
    })
}
