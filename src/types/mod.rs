use crate::terminated_array;
use core::{
    fmt::{self, Debug},
    ptr::NonNull,
};

mod vmt;
pub use vmt::*;

/// Windows datatypes
#[cfg(windows)]
pub mod win;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;

/// **Non null** C-Like zero terminated string
/// # Safety
/// This struct expects a non null ptr. So if you are not sure if that's the case
/// you may use [`Option<CStr>`] because this struct will benefit from non null optimizions.
/// Has the same aliging and size as [`NonNull<i8>`]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CStr {
    ptr: NonNull<i8>,
}

impl CStr {
    /// Creates new [`CStr`].
    #[inline]
    pub fn from_ptr(ptr: NonNull<i8>) -> Self {
        Self { ptr }
    }

    /// Converts [`CStr`] to raw pointer.
    #[inline]
    pub fn as_ptr(&self) -> *const i8 {
        self.ptr.as_ptr() as _
    }

    /// Converts [`CStr`] to string slice.
    /// # Safety
    /// * [`CStr`] must point to a valid UTF-8 sequence.
    #[inline]
    pub unsafe fn as_str<'a>(&self) -> &'a str {
        core::str::from_utf8_unchecked(terminated_array::<u8>(self.ptr.as_ptr() as _, 0))
    }

    /// Converts [`CStr`] into signed byte slice.
    /// # Safety
    /// * [`CStr`] must be a valid pointer.
    #[inline]
    pub unsafe fn as_slice<'a>(&self) -> &'a [i8] {
        terminated_array(self.ptr.as_ptr() as _, 0)
    }

    /// Counts the number of bytes in a string.
    /// # Safety
    /// * [`CStr`] must be a valid pointer.
    #[inline]
    pub unsafe fn len(&self) -> usize {
        terminated_array(self.ptr.as_ptr(), 0).len()
    }

    /// Checks if the string is empty.
    /// # Safety
    /// * [`CStr`] must be a valid pointer.
    #[inline]
    pub unsafe fn is_empty(&self) -> bool {
        *self.ptr.as_ptr() == 0
    }
}

impl Debug for CStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe { write!(f, "{:?}", self.as_str()) }
    }
}

/// Basic information about module
#[derive(Debug, Clone, Copy)]
#[cfg(feature = "internal")]
pub struct ModuleBasicInfo {
    /// Module's base
    pub base: *const u8,
    /// Module's size
    pub size: usize,
}

/// More information about module
#[derive(Debug, Clone)]
#[cfg(all(feature = "alloc", feature = "internal"))]
pub struct ModuleAdvancedInfo {
    /// Module's base
    pub base: *const u8,
    /// Module's size
    pub size: usize,
    /// Module's name
    pub name: String,
    /// Module's full path
    pub path: String,
}
