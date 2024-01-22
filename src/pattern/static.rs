#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;

use super::ByteMatch;
use crate::Matcher;

#[test]
fn test_char_to_u8() {
    assert_eq!(single('0'), 0x0);
    assert_eq!(single('9'), 0x9);
    assert_eq!(single('A'), 0xA);
    assert_eq!(single('F'), 0xF);
}

const fn single(c: char) -> u8 {
    match c {
        'a'..='f' => 10 + (c as u8 - b'a'),
        'A'..='F' => 10 + (c as u8 - b'A'),
        '0'..='9' => c as u8 - b'0',
        _ => panic!("Invalid pattern"),
    }
}

/// Represents a staticly built sequence of bytes to match against.
/// ```
/// # use memflex::{ida_pat, peid_pat, code_pat};
/// let data = b"\x11\x22\x33";
/// let ida = ida_pat!("11 ? 33");
/// let peid = peid_pat!("11 ?? 33");
/// let code = code_pat!(b"\x11\x00\x33", "x?x");
/// assert!(ida.matches(data) && peid.matches(data) && code.matches(data));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pattern<const N: usize>(pub(crate) [ByteMatch; N]);

#[allow(clippy::wrong_self_convention)]
impl<const N: usize> Pattern<N> {
    #[cfg(feature = "alloc")]
    fn to_ida_peid_style(&self, peid: bool) -> String {
        self.0
            .iter()
            .map(|m| match m {
                ByteMatch::Exact(b) => alloc::format!("{b:02X}"),
                ByteMatch::Any => if peid { "??" } else { "?" }.into(),
            })
            .collect::<alloc::vec::Vec<_>>()
            .join(" ")
    }

    /// Converts pattern to IDA style string.
    #[cfg(feature = "alloc")]
    pub fn to_ida_style(&self) -> String {
        self.to_ida_peid_style(false)
    }

    /// Converts pattern to PEID style string.
    #[cfg(feature = "alloc")]
    pub fn to_peid_style(&self) -> String {
        self.to_ida_peid_style(true)
    }

    /// Converts pattern to code style string, returing pattern and mask.
    #[cfg(feature = "alloc")]
    pub fn to_code_style(&self) -> (String, String) {
        self.0
            .iter()
            .map(|m| match m {
                ByteMatch::Exact(b) => (alloc::format!("\\x{b:02X}"), "x"),
                ByteMatch::Any => ("?".into(), "?"),
            })
            .unzip::<_, _, String, String>()
    }

    /// Checks if pattern matches byte slice.
    /// ```
    /// # use memflex::ida_pat;
    /// let data = b"\x11\x22\x33";
    /// let pat = ida_pat!("11 ? 33");
    /// assert!(pat.matches(data));
    /// ```
    pub const fn matches(&self, data: &[u8]) -> bool {
        if data.len() != N {
            return false;
        }

        let mut i = 0;
        while i < data.len() {
            if !self.0[i].matches(data[i]) {
                return false;
            }

            i += 1;
        }
        true
    }

    const fn from_ida_peid_style(pat: &'static str, peid: bool) -> Pattern<N> {
        let mut out = [ByteMatch::Any; N];

        let mut i = 0;
        let mut j = 0;

        while i < pat.len() - 1 {
            let c1 = pat.as_bytes()[i];
            let c2 = pat.as_bytes()[i + 1];

            if c1 == b'?' && c2 == if peid { b'?' } else { b' ' } {
                out[j] = ByteMatch::Any;
                j += 1;
                i += 2 + (peid as usize);
                continue;
            } else if c2 == b' ' {
                i += 2;
                continue;
            }

            let byte = single(c1 as char) * 0x10 + single(c2 as char);
            out[j] = ByteMatch::Exact(byte);
            j += 1;
            i += 1;
        }

        Self(out)
    }

    /// Creates pattern from IDA style string.
    /// ```
    /// # use memflex::{ida_pat, peid_pat} ;
    /// // Pattern parsing is a contant call and happens at compile time.
    /// let ida = ida_pat!("13 ? D1");
    /// let data = b"\x13\x01\xD1";
    /// assert!(ida.matches(data));
    #[inline]
    pub const fn from_ida_style(pat: &'static str) -> Pattern<N> {
        Self::from_ida_peid_style(pat, false)
    }

    /// Creates pattern from PEID style string.
    /// ```
    /// # use memflex::{ida_pat, peid_pat} ;
    /// // Pattern parsing is a contant call and happens at compile time.
    /// let peid = peid_pat!("13 ?? D1");
    /// let data = b"\x13\x01\xD1";
    /// assert!(peid.matches(data));
    #[inline]
    pub const fn from_peid_style(pat: &'static str) -> Pattern<N> {
        Self::from_ida_peid_style(pat, true)
    }

    /// Creates pattern from code style strings.
    /// ```
    /// # use memflex::code_pat;
    /// let pat = code_pat!(b"\x11\x55\xE2", "x?x");
    /// let data = b"\x11\x01\xE2";
    /// assert!(pat.matches(data));
    /// ```
    pub const fn from_code_style(pat: &'static [u8], mask: &'static str) -> Pattern<N> {
        let mut j = 0;
        while j < mask.len() {
            if mask.as_bytes()[j] != b'x' && mask.as_bytes()[j] != b'?' {
                panic!("Mask must only contain only `x` or `?`");
            }

            j += 1;
        }

        let mut out = [ByteMatch::Any; N];

        let mut i = 0;
        while i < mask.len() {
            let mask = mask.as_bytes()[i];

            match mask {
                b'x' => out[i] = ByteMatch::Exact(pat[i]),
                b'?' => out[i] = ByteMatch::Any,
                _ => panic!("Invalid pattern"),
            }

            i += 1;
        }

        Self(out)
    }
}

impl<const N: usize> Matcher for Pattern<N> {
    fn matches(&self, seq: &[u8]) -> bool {
        self.matches(seq)
    }

    fn len(&self) -> usize {
        N
    }
}

/// Generates a pattern from IDA style string.
/// ```
/// # use memflex::ida_pat;
/// // Pattern parsing is a contant call and happens at compile time.
/// let pat = ida_pat!("13 ? D1");
/// ```
#[macro_export]
macro_rules! ida_pat {
    [
        $pat:expr
    ] => {
        $crate::Pattern::<{ $crate::__ida_peid_count($pat, false) }>::from_ida_style($pat)
    };
}

/// Generates a pattern from PEID style string.
/// ```
/// # use memflex::peid_pat;
/// // Pattern parsing is a contant call and happens at compile time.
/// let pat = peid_pat!("13 ?? D1");
/// ```
#[macro_export]
macro_rules! peid_pat {
    [
        $pat:expr
    ] => {
        $crate::Pattern::<{ $crate::__ida_peid_count($pat, true) }>::from_peid_style($pat)
    };
}

#[doc(hidden)]
pub const fn __ida_peid_count(pat: &'static str, peid: bool) -> usize {
    let mut i = 0;
    let mut j = 0;

    while i < pat.len() - 1 {
        let c1 = pat.as_bytes()[i];
        let c2 = pat.as_bytes()[i + 1];

        if c1 == b'?' && c2 == if peid { b'?' } else { b' ' } {
            j += 1;
            i += 2 + (peid as usize);
        } else if c2 == b' ' {
            i += 2;
            continue;
        }

        j += 1;
        i += 1;
    }

    j
}

/// Generates a pattern from code style strings.
/// ```
/// # use memflex::code_pat;
/// // Pattern parsing is a contant call and happens at compile time.
/// let pat = code_pat!(b"\x11\x55\xE2", "x?x");
/// ```
#[macro_export]
macro_rules! code_pat {
    [
        $pat:literal $(,)? $mask:literal
    ] => {
        $crate::Pattern::<{ $mask.len() }>::from_code_style($pat, $mask)
    };
}
