use crate::{description::CompiledDescription, error::ProblemError, Solution, utils::FieldGrid};

pub mod naive;

pub trait ProblemSolver: Default {
    fn solve(&mut self, description: &CompiledDescription) -> Result<FieldGrid, ProblemError>;
}

#[cfg(feature = "wasm")]
mod wasm;

#[cfg(feature = "wasm")]
pub fn create_builder<D>() -> wasm::WasmProblemBuilder<D> {
    wasm::WasmProblemBuilder::builder()
}

#[cfg(feature = "z3feature")]
mod z3solver;

#[cfg(feature = "z3feature")]
pub fn create_builder<D>() -> native::NativeProblemBuilder<D> {
    native::NativeProblemBuilder::builder()
}
