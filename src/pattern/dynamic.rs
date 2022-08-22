use super::{ByteMatch, sealed};
use crate::{Pattern, Matcher};

/// Dynamic pattern. Same as [`crate::pattern::Pattern`] but requires allocating.
pub struct DynPattern(pub(crate) Vec<ByteMatch>);

impl DynPattern {
    /// Checks if the `data` matches the pattern.
    #[inline]
    pub fn matches(&self, data: &[u8]) -> bool {
        self.0
            .iter()
            .zip(data.iter())
            .all(|(a, b)| a.matches(*b))
    }
}

impl<const N: usize> From<Pattern<N>> for DynPattern {
    fn from(p: Pattern<N>) -> Self {
        Self(p.0.into())
    }
}

impl sealed::Sealed for DynPattern { }

impl Matcher for DynPattern {
    fn matches(&self, seq: &[u8]) -> bool {
        self.matches(seq)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}
