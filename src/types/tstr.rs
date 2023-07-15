use crate::terminated_array;
use core::{
    fmt::{self, Debug},
    ptr::NonNull,
};

/// Non null pointer to C zero terminated string
/// # Safety
/// This struct expects a non null ptr. So if you are not sure if that's the case
/// you may use [`Option<TStr>`] because this struct will benefit from non null optimizions.
/// Has the same aliging and size as [`NonNull<i8>`]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct TStr {
    ptr: NonNull<i8>,
}

impl TStr {
    /// Creates new [`TStr`].
    #[inline]
    pub fn from_ptr(ptr: NonNull<i8>) -> Self {
        Self { ptr }
    }

    /// Converts [`TStr`] to raw pointer.
    #[inline]
    pub fn as_ptr(&self) -> *const i8 {
        self.ptr.as_ptr() as _
    }

    /// Converts [`TStr`] to string slice.
    /// # Safety
    /// * [`TStr`] must point to a valid UTF-8 sequence.
    #[inline]
    pub unsafe fn as_str<'a>(&self) -> &'a str {
        core::str::from_utf8_unchecked(terminated_array::<u8>(self.ptr.as_ptr() as _, 0))
    }

    /// Converts [`TStr`] into signed byte slice.
    /// # Safety
    /// * [`TStr`] must be a valid pointer.
    #[inline]
    pub unsafe fn as_slice<'a>(&self) -> &'a [i8] {
        terminated_array(self.ptr.as_ptr() as _, 0)
    }

    /// Counts the number of bytes in a string.
    /// # Safety
    /// * [`TStr`] must be a valid pointer.
    #[inline]
    pub unsafe fn len(&self) -> usize {
        terminated_array(self.ptr.as_ptr(), 0).len()
    }

    /// Checks if the string is empty.
    /// # Safety
    /// * [`TStr`] must be a valid pointer.
    #[inline]
    pub unsafe fn is_empty(&self) -> bool {
        *self.ptr.as_ptr() == 0
    }
}

impl Debug for TStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe { write!(f, "{:?}", self.as_str()) }
    }
}

/// Generates C-like terminated string with the type of [`crate::types::TStr`]
/// ```
/// # use memflex::{tstr, terminated_array};
/// # unsafe {
/// assert_eq!(tstr!("123").as_str(), "123");
/// # }
/// ```
#[macro_export]
macro_rules! tstr {
    ( $($tt:tt)* ) => {
        $crate::types::TStr::from_ptr(unsafe { core::ptr::NonNull::new_unchecked(core::concat!($($tt)*, "\x00").as_ptr() as _) })
    }
}
