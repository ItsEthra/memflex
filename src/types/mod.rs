use crate::internal::terminated_array;
use core::ptr::NonNull;

/// **Non null** C-Like zero terminated string
/// # Safety
/// This macro expects a non null ptr. So if you are not sure if that's the case
/// you may use [`Option<CStr>`] because this struct will benefit from non null optimizions.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CStr {
    /// Pointer to string data
    pub ptr: NonNull<i8>
}

impl CStr {
    /// Creates new [`CStr`].
    pub fn from_ptr(ptr: NonNull<i8>) -> Self {
        Self { ptr }
    }
    
    /// Converts [`CStr`] into raw pointer..
    pub fn into_ptr(self) -> *const i8 {
        self.ptr.as_ptr() as _    
    }

    /// Converts [`CStr`] to string slice.
    /// # Safety
    /// * [`CStr`] must point to a valid UTF-8 sequence.
    pub unsafe fn as_str<'a>(&self) -> &'a str {
        core::str::from_utf8_unchecked(
            terminated_array::<u8>(self.ptr.as_ptr() as _, 0)
        )
    }

    /// Converts [`CStr`] into signed byte slice.
    /// # Safety
    /// * [`CStr`] must be a valid pointer.
    pub unsafe fn as_slice<'a>(&self) -> &'a [i8] {
        terminated_array(self.ptr.as_ptr() as _, 0)
    }
}