#[derive(Debug, Clone, Copy)]
enum ByteMatch {
    Exact(u8),
    Any
}

// impl ByteMatch {
//     #[inline]
//     pub fn matches(self, byte: u8) -> bool {
//         match self {
//             ByteMatch::Exact(b) => b == byte,
//             ByteMatch::Any => true,
//         }
//     }
// }

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
        _ => panic!("Invalid pattern")
    }
}

/// Represents a sequence of bytes to match against.
#[derive(Debug)]
pub struct Pattern<const N: usize>([ByteMatch; N]);
impl<const N: usize> Pattern<N> {
    /// Creates pattern from IDA style string
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

        Pattern(out)
    }
}

/// Generates a pattern from IDA style string.
/// ```
/// # use memflex::ida_pat;
/// 
/// // They all are actually constant calls so all transformations happen at compile time
/// let pat = ida_pat!("13 ? D1");
/// ```
#[macro_export]
macro_rules! ida_pat {
    [
        $pat:literal
    ] => {
        $crate::pattern::Pattern::<{ $crate::pattern::__ida_peid_count($pat, false) }>::from_ida_peid_style($pat, false)
    };
}

/// Generates a pattern from PEID style string.
/// ```
/// # use memflex::peid_pat;
/// 
/// // They all are actually constant calls so all transformations happen at compile time
/// let pat = peid_pat!("13 ?? D1");
/// ```
#[macro_export]
macro_rules! peid_pat {
    [
        $pat:literal
    ] => {
        $crate::pattern::Pattern::<{ $crate::pattern::__ida_peid_count($pat, true) }>::from_ida_peid_style($pat, true)
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