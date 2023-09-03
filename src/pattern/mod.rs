mod r#static;
pub use r#static::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ByteMatch {
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

/// Trait for generalizing static & dynamic memory patterns.
#[allow(clippy::len_without_is_empty)]
pub trait Matcher {
    /// Matches byte sequence agains the pattern
    fn matches(&self, seq: &[u8]) -> bool;

    /// Size of the pattern
    fn len(&self) -> usize;
}

impl<'a> Matcher for &'a [u8] {
    fn matches(&self, seq: &[u8]) -> bool {
        seq.len() == self.len() && self.iter().zip(seq.iter()).all(|(a, b)| a.eq(b))
    }

    fn len(&self) -> usize {
        (*self as &[u8]).len()
    }
}
