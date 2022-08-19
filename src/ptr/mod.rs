mod r#const;
pub use r#const::*;
mod r#mut;
pub use r#mut::*;

use core::ops::{ControlFlow, FromResidual, Try};

/// Function flow when checking pointers
pub enum Flow<T> {
    /// Pointer was null, abort
    Null,
    /// All good, continue
    Done(T),
}

impl<T> Try for Flow<T> {
    type Output = T;
    type Residual = Flow<T>;

    fn from_output(output: Self::Output) -> Self {
        Self::Done(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Flow::Done(v) => ControlFlow::Continue(v),
            Flow::Null => ControlFlow::Break(Flow::Null),
        }
    }
}

impl<A, B> FromResidual<Flow<A>> for Flow<B> {
    fn from_residual(_: Flow<A>) -> Self {
        Flow::Null
    }
}
