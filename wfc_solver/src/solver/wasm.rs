use std::marker::PhantomData;

use super::ProblemBuilder;

pub struct WasmProblemBuilder<D> {
    _data: PhantomData<D>,
}

impl<D> ProblemBuilder for WasmProblemBuilder<D> {
    type DataId = D;

    fn builder() -> Self {
        todo!()
    }

    fn solve(&self) -> Result<crate::Solution<Self::DataId>, super::ProblemError> {
        todo!()
    }
}
