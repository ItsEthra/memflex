#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::{FromUtf16Error, String};

use core::{fmt::Debug, mem::size_of, slice::from_raw_parts};

/// Unicode string in UTF-16 format
#[derive(Debug)]
#[repr(C)]
pub struct UnicodeString {
    len: u16,
    cap: u16,
    buf: *const u16,
}

impl UnicodeString {
    /// Creates new unicode string from its components.
    /// `len`, `cap` must be in bytes and `buf` must be available to read.
    #[inline]
    pub const fn new(len: u16, cap: u16, buf: *const u16) -> Self {
        Self { len, cap, buf }
    }

    /// Returns the length in UTF-16 characters of the string,
    /// although it returns [`usize`], maximum value for it is `u16::MAX`.
    #[inline]
    pub const fn len(&self) -> usize {
        self.len as usize / size_of::<u16>()
    }

    /// Checks if the string is empty
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the length in bytes of the string,
    /// although it returns [`usize`], maximum value for it is `u16::MAX`.
    #[inline]
    pub const fn bytes_len(&self) -> usize {
        self.len as usize
    }

    /// Returns the allocated size or capacity of the string in bytes,
    /// although it returns [`usize`], maximum value for it is `u16::MAX`.
    #[inline]
    pub const fn capacity(&self) -> usize {
        self.cap as usize
    }

    /// Checks if the `buffer` pointer is zero.
    #[inline]
    pub fn is_null(&self) -> bool {
        self.buf.is_null()
    }

    /// Converts unicode string to a UTF-16 byte slice
    /// # Safety
    /// * [`UnicodeString`] must be a valid pointer
    #[inline]
    pub unsafe fn as_slice<'a>(&self) -> &'a [u16] {
        from_raw_parts(self.buf, self.len())
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
    #[inline]
    pub unsafe fn is_ascii(&self) -> bool {
        self.as_slice().iter().all(|b| *b < 128)
    }

    /// Creates an iterator over all string character if they are ascii.
    /// # Safety
    /// * [`UnicodeString`] must be a valid pointer
    /// # Panics
    /// * If string contains non-ascii characters.
    #[inline]
    pub unsafe fn ascii(&self) -> impl Iterator<Item = char> {
        assert!(self.is_ascii());

        self.as_slice().iter().map(|b| *b as u8 as char)
    }

    /// Creates an iterator over valid UTF-16 string characters, invalid characters are replaced
    /// with replacement character.
    /// # Safety
    /// * [`UnicodeString`] must be a valid pointer
    #[inline]
    pub unsafe fn utf16(&self) -> impl Iterator<Item = char> {
        char::decode_utf16(self.as_slice().iter().cloned()).map(|ch| match ch {
            Ok(c) => c,
            Err(_) => char::REPLACEMENT_CHARACTER,
        })
    }
}

impl PartialEq for UnicodeString {
    fn eq(&self, other: &Self) -> bool {
        if self.is_null() && other.is_null() {
            return true;
        }

        if self.bytes_len() != other.bytes_len() {
            false
        } else {
            unsafe { self.as_slice() == other.as_slice() }
        }
    }
}

/// A literal Unicode string.
#[macro_export]
macro_rules! unicode_string {
    ($str:expr) => {
        $crate::types::win::UnicodeString::new(
            ($str.len() * 2) as _,
            ($str.len() * 2) as _,
            $crate::obfstr::wide!($str).as_ptr() as _,
        )
    };
}
