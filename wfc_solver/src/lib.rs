use description::{CompiledDescription, ProblemDescription};
use error::ProblemError;
use solver::{naive::NaiveSolver, ProblemSolver};
use utils::FieldGrid;

pub mod error;
pub mod solver;
pub mod utils;

pub mod description;

pub struct Solution<D> {
    grid: FieldGrid,
    description: ProblemDescription<D>,
    compiled: CompiledDescription,
}

pub fn solve<D>(description: ProblemDescription<D>) -> Result<Solution<D>, ProblemError> {
    let compiled = description.compile();
    let mut solver = NaiveSolver::default();

    let grid = solver.solve(&compiled)?;

    Ok(Solution {
        grid,
        description,
        compiled,
    })
}
