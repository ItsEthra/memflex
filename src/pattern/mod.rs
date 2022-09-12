mod r#static;
pub use r#static::*;
mod dynamic;
pub use dynamic::*;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
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

mod sealed {
    pub trait Sealed {}
}

/// Trait for generalizing static & dynamic memory patterns.
pub trait Matcher: sealed::Sealed {
    /// Matches byte sequence agains the pattern
    fn matches(&self, seq: &[u8]) -> bool;

    /// Size of the pattern
    fn size(&self) -> usize;
}

impl<'a> sealed::Sealed for &'a [u8] {}

impl<'a> Matcher for &'a [u8] {
    fn matches(&self, seq: &[u8]) -> bool {
        self.iter().zip(seq.iter()).all(|(a, b)| a.eq(b))
    }

    fn size(&self) -> usize {
        self.len()
    }
}
