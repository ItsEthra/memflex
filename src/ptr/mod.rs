mod r#const;
pub use r#const::*;
mod r#mut;
pub use r#mut::*;

use core::ops::{ControlFlow, FromResidual, Try};

/// Error trait that allows creating an error signaling null ptr
pub trait NullError {
    /// Method for creating an error type signaling null ptr.
    fn null() -> Self;
}

/// Function flow when checking pointers
pub enum Flow<T> {
    /// Pointer was null, abort
    Null,
    /// All good, continue
    Done(T),
}

impl<T> Flow<T> {
    /// Converts [`Flow`] to [`Result`].
    #[inline]
    pub fn ok<E: NullError>(self) -> Result<T, E> {
        match self {
            Flow::Done(v) => Ok(v),
            Flow::Null => Err(E::null()),
        }
    }

    /// Converts [`Flow`] to [`Option`].
    #[inline]
    pub fn some(self) -> Option<T> {
        match self {
            Flow::Done(v) => Some(v),
            Flow::Null => None,
        }
    }

    /// Checks if the flow is `null`.
    #[inline]
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Unwraps return value
    /// # Panics
    /// If the flow was `null`
    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Flow::Done(v) => v,
            Flow::Null => unreachable!(),
        }
    }
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
