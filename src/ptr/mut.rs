use core::{
    marker::PhantomData,
    ops::{ControlFlow, Try, FromResidual},
    ptr::NonNull,
};
use crate::Flow;

/// Mutable pointer with a static lifetime
pub type PtrMutStatic<T> = PtrMut<'static, T>;

/// Mutable pointer that can be checked for null with `?` operator.
#[derive(Debug, PartialEq, Eq)]
pub enum PtrMut<'a, T> {
    /// Valid ptr
    Valid(NonNull<T>, PhantomData<&'a T>),
    /// Null ptr
    Null,
}

#[test]
fn test_ptr_size() {
    use core::mem::size_of as sof;
    assert_eq!(sof::<PtrMut<()>>(), sof::<usize>());
}

impl<'a, T> PtrMut<'a, T> {
    /// Creates new pointer from reference
    pub fn new(r: &'a mut T) -> Self {
        Self::Valid(
            unsafe { NonNull::new_unchecked(r as *mut _) },
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

impl<'a, T> Try for PtrMut<'a, T> {
    type Output = &'a mut T;
    type Residual = Flow<()>;

    fn from_output(output: Self::Output) -> Self {
        Self::new(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            PtrMut::Valid(ptr, _) => ControlFlow::Continue(unsafe { &mut *ptr.as_ptr() }),
            PtrMut::Null => ControlFlow::Break(Flow::Null),
        }
    }
}

impl<'a, T> From<Option<NonNull<T>>> for PtrMut<'a, T> {
    fn from(val: Option<NonNull<T>>) -> Self {
        if let Some(ptr) = val {
            Self::Valid(ptr, PhantomData)
        } else {
            Self::Null
        }
    }
}

impl<'a, T> From<NonNull<T>> for PtrMut<'a, T> {
    fn from(val: NonNull<T>) -> Self {
        Self::Valid(val, PhantomData)
    }
}

impl<'a, T> From<Box<T>> for PtrMut<'a, T> {
    fn from(val: Box<T>) -> Self {
        unsafe { Self::Valid(NonNull::new_unchecked(Box::into_raw(val)), PhantomData) }
    }
}

impl<'a, T> From<&'a mut T> for PtrMut<'a, T> {
    fn from(val: &'a mut T) -> Self {
        unsafe { Self::Valid(NonNull::new_unchecked(val as *const _ as _), PhantomData) }
    }
}

impl<'a, T> From<*mut T> for PtrMut<'a, T> {
    fn from(val: *mut T) -> Self {
        if val.is_null() {
            Self::Null
        } else {
            unsafe { Self::Valid(NonNull::new_unchecked(val), PhantomData) }
        }
    }
}

impl<'a, T, V> FromResidual<Flow<V>> for PtrMut<'a, T> {
    fn from_residual(_: Flow<V>) -> Self {
        Self::Null
    }
}