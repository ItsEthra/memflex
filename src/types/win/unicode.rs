#[cfg(feature = "alloc")]
extern crate alloc;
use core::{slice::from_raw_parts, mem::size_of, fmt::Debug};

#[cfg(feature = "alloc")]
use alloc::string::{String, FromUtf16Error};

/// Unicode string in UTF-16 format
#[derive(Debug)]
#[repr(C)]
pub struct UnicodeString {
    length: u16,
    capacity: u16,
    buffer: *const u16
}

impl UnicodeString {
    /// Returns the length in UTF-16 characters of the string,
    /// although it returns [`usize`], maximum value for it is `u16::MAX`.
    #[inline]
    pub const fn len(&self) -> usize {
        self.length as usize / size_of::<u16>()
    }

    /// Returns the length in bytes of the string,
    /// although it returns [`usize`], maximum value for it is `u16::MAX`.
    #[inline]
    pub const fn bytes_len(&self) -> usize {
        self.length as usize
    }

    /// Returns the allocated size or capacity of the string in bytes,
    /// although it returns [`usize`], maximum value for it is `u16::MAX`.
    #[inline]
    pub const fn capacity(&self) -> usize {
        self.capacity as usize
    }

    /// Checks if the `buffer` pointer is zero.
    #[inline]
    pub fn is_null(&self) -> bool {
        self.buffer.is_null()
    }

    /// Converts unicode string to a UTF-16 byte slice
    /// # Safety
    /// * [`UnicodeString`] must be a valid pointer
    #[inline]
    pub unsafe fn as_slice<'a>(&self) -> &'a [u16] {
        from_raw_parts(self.buffer, self.len())
    }

    /// Tries to convert unicode string to UTF-8 without taking ownership.
    /// # Safety
    /// * [`UnicodeString`] must be a valid pointer
    #[cfg(feature = "alloc")]
    pub unsafe fn to_string(&self) -> Result<String, FromUtf16Error> {
        String::from_utf16(self.as_slice())
    }

    /// Checks if string contains only characters that can be represented with ASCII
    /// # Safety
    /// * [`UnicodeString`] must be a valid pointer
    pub unsafe fn is_ascii(&self) -> bool {
        self.as_slice()
            .iter()
            .map(|b| char::decode_utf16([*b]))
            .flatten()
            .all(|c| c.is_ok())
    }
}