use std::marker::PhantomData;

use crate::Solution;

use super::{ProblemBuilder, ProblemError};
use z3::*;

#[derive(Debug)]
pub struct NativeProblemBuilder<D> {
    context: Context,
    _phantom: PhantomData<D>,
}

impl<D> ProblemBuilder for NativeProblemBuilder<D> {
    fn builder() -> Self {
        let mut config = Config::new();
        config.set_proof_generation(true);

        let context = Context::new(&config);
        Self {
            context,
            _phantom: Default::default(),
        }
    }

    fn solve(&self) -> Result<Solution<D>, ProblemError> {
        let context = &self.context;
        let solver = Solver::new(&self.context);

        let proof = match solver.check() {
            z3::SatResult::Unsat => return Err(ProblemError::Unsatisfiable),
            z3::SatResult::Unknown => return Err(ProblemError::Unsatisfiable),
            z3::SatResult::Sat => solver.get_proof().unwrap(),
        };

        Ok(Solution { tiles: Vec::new() })
    }

    type DataId = D;
}
