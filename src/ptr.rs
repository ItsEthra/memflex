use core::{
    marker::PhantomData,
    ops::{ControlFlow, FromResidual, Try},
    ptr::NonNull,
};

/// Pointer with a static lifetime
pub type PtrStatic<T> = Ptr<'static, T>;

/// Pointer that can be checked for null with `?` operator.
pub enum Ptr<'a, T> {
    /// Valid ptr
    Valid(NonNull<T>, PhantomData<&'a T>),
    /// Null ptr
    Null,
}

impl<'a, T> Ptr<'a, T> {
    /// Creates new pointer from reference
    pub fn new(r: &'a T) -> Self {
        Self::Valid(
            unsafe { NonNull::new_unchecked(r as *const _ as _) },
            PhantomData,
        )
    }

    /// Creates new pointer from reference with arbitrary lifetime
    pub unsafe fn new_unchecked<'b>(r: &'b T) -> Self {
        Self::Valid(NonNull::new_unchecked(r as *const _ as _), PhantomData)
    }

    /// Creates new null pointer
    pub fn null() -> Self {
        Self::Null
    }
}

impl<'a, T> From<Box<T>> for Ptr<'a, T> {
    fn from(val: Box<T>) -> Self {
        unsafe { Self::Valid(NonNull::new_unchecked(Box::into_raw(val)), PhantomData) }
    }
}

impl<'a, T> Try for Ptr<'a, T> {
    type Output = &'a T;
    type Residual = Flow<()>;

    fn from_output(output: Self::Output) -> Self {
        Self::new(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Ptr::Valid(ptr, _) => ControlFlow::Continue(unsafe { &*ptr.as_ptr() }),
            Ptr::Null => ControlFlow::Break(Flow::Null),
        }
    }
}

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

impl<'a, T, V> FromResidual<Flow<V>> for Ptr<'a, T> {
    fn from_residual(residual: Flow<V>) -> Self {
        match residual {
            Flow::Null => Self::Null,
            Flow::Done(_) => todo!(),
        }
    }
}

impl<A, B> FromResidual<Flow<A>> for Flow<B> {
    fn from_residual(_: Flow<A>) -> Self {
        Flow::Null
    }
}

#[test]
fn test_ptr_size() {
    use core::mem::size_of as sof;
    assert_eq!(sof::<Ptr<()>>(), sof::<usize>());
}
