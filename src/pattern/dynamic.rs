use super::{sealed, ByteMatch};
use crate::{Matcher, Pattern};

/// Dynamic pattern. Same as [`crate::pattern::Pattern`] but requires allocating and can be build at runtime.
pub struct DynPattern(pub(crate) Vec<ByteMatch>);

impl DynPattern {
    /// Checks if the `data` matches the pattern.
    #[inline]
    pub fn matches(&self, data: &[u8]) -> bool {
        self.0.iter().zip(data.iter()).all(|(a, b)| a.matches(*b))
    }

    fn to_ida_peid_style(&self, peid: bool) -> String {
        self.0
            .iter()
            .map(|m| match m {
                ByteMatch::Exact(b) => format!("{b:02X}"),
                ByteMatch::Any => if peid { "??" } else { "?" }.into(),
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Converts pattern to IDA style string.
    pub fn to_ida_style(&self) -> String {
        self.to_ida_peid_style(false)
    }

    /// Converts pattern to PEID style string.
    pub fn to_peid_style(&self) -> String {
        self.to_ida_peid_style(true)
    }

    /// Converts pattern to code style string, returing pattern and mask.
    pub fn to_code_style(&self) -> (String, String) {
        self.0
            .iter()
            .map(|m| match m {
                ByteMatch::Exact(b) => (format!("\\x{b:02X}"), "x"),
                ByteMatch::Any => ("?".into(), "?"),
            })
            .unzip::<_, _, String, String>()
    }
}

impl<const N: usize> From<Pattern<N>> for DynPattern {
    fn from(p: Pattern<N>) -> Self {
        Self(p.0.into())
    }
}

impl<'a> From<&'a [u8]> for DynPattern {
    fn from(slice: &'a [u8]) -> Self {
        Self(slice.iter().map(|b| ByteMatch::Exact(*b)).collect())
    }
}

impl sealed::Sealed for DynPattern {}

impl Matcher for DynPattern {
    fn matches(&self, seq: &[u8]) -> bool {
        self.matches(seq)
    }

    fn size(&self) -> usize {
        self.0.len()
    }
}
