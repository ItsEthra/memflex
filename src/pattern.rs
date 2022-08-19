#[derive(Debug, Clone, Copy, PartialEq, Hash)]
enum ByteMatch {
    Exact(u8),
    Any,
}

impl ByteMatch {
    #[inline]
    pub const fn matches(self, byte: u8) -> bool {
        match self {
            ByteMatch::Exact(b) => b == byte,
            ByteMatch::Any => true,
        }
    }
}

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

/// Represents a sequence of bytes to match against.
/// ```
/// # use memflex::{ida_pat, peid_pat, code_pat};
/// let data = b"\x11\x22\x33";
/// let ida = ida_pat!("11 ? 33");
/// let peid = peid_pat!("11 ?? 33");
/// let code = code_pat!(b"\x11\x00\x33", "x?x");
/// assert!(ida.matches(data) && peid.matches(data) && code.matches(data));
/// ```
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Pattern<const N: usize>([ByteMatch; N]);
impl<const N: usize> Pattern<N> {
    /// Checks if pattern matches byte slice
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

    /// Creates pattern from IDA or PEID style string
    /// ```
    /// # use memflex::{ida_pat, peid_pat} ;
    /// // They are actually constant calls so all transformations happen at compile time
    /// let ida = ida_pat!("13 ? D1");
    /// let peid = ida_pat!("13 ? D1");
    /// let data = b"\x13\x01\xD1";
    /// assert!(ida.matches(data));
    /// assert!(peid.matches(data));
    pub const fn from_ida_peid_style(pat: &'static str, peid: bool) -> Pattern<N> {
        let mut out = [ByteMatch::Any; N];

        let mut i = 0;
        let mut j = 0;

        while i < pat.len() - 1 {
            let c1 = unsafe { *pat.as_ptr().add(i) };
            let c2 = unsafe { *pat.as_ptr().add(i + 1) };

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

    /// Creates pattern from code style strings
    /// ```
    /// # use memflex::code_pat;
    /// let pat = code_pat!(b"\x11\x55\xE2", "x?x");
    /// let data = b"\x11\x01\xE2";
    /// assert!(pat.matches(data));
    /// ```
    pub const fn from_code_style(pat: &'static [u8], mask: &'static str) -> Pattern<N> {
        let mut out = [ByteMatch::Any; N];

        let mut i = 0;
        while i < mask.len() {
            let mask = unsafe { *mask.as_ptr().add(i) } as char;

            match mask {
                'x' => out[i] = ByteMatch::Exact(pat[i]),
                '?' => out[i] = ByteMatch::Any,
                _ => panic!("Invalid pattern"),
            }

            i += 1;
        }

        Self(out)
    }
}

/// Generates a pattern from IDA style string.
/// ```
/// # use memflex::ida_pat;
/// // Pattern parsing is a contant call and happens at compile time
/// let pat = ida_pat!("13 ? D1");
/// ```
#[macro_export]
macro_rules! ida_pat {
    [
        $pat:literal
    ] => {
        $crate::Pattern::<{ $crate::__ida_peid_count($pat, false) }>::from_ida_peid_style($pat, false)
    };
}

/// Generates a pattern from PEID style string.
/// ```
/// # use memflex::peid_pat;
/// // Pattern parsing is a contant call and happens at compile time
/// let pat = peid_pat!("13 ?? D1");
/// ```
#[macro_export]
macro_rules! peid_pat {
    [
        $pat:literal
    ] => {
        $crate::Pattern::<{ $crate::__ida_peid_count($pat, true) }>::from_ida_peid_style($pat, true)
    };
}

#[doc(hidden)]
pub const fn __ida_peid_count(pat: &'static str, peid: bool) -> usize {
    let mut i = 0;
    let mut j = 0;

    while i < pat.len() - 1 {
        let c1 = unsafe { *pat.as_ptr().add(i) };
        let c2 = unsafe { *pat.as_ptr().add(i + 1) };

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
/// // Pattern parsing is a contant call and happens at compile time
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
