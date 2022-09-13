mod vmt;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

pub use vmt::*;
mod tstr;
pub use tstr::*;

/// Windows datatypes
#[cfg(windows)]
pub mod win;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;

/// Basic information about module
#[derive(Debug, Clone, Copy)]
#[cfg(feature = "internal")]
pub struct ModuleBasicInfo {
    /// Module's base
    pub base: *const u8,
    /// Module's size
    pub size: usize,
}

/// More information about module
#[derive(Debug, Clone)]
#[cfg(all(feature = "alloc", feature = "internal"))]
pub struct ModuleAdvancedInfo {
    /// Module's base
    pub base: *const u8,
    /// Module's size
    pub size: usize,
    /// Module's name
    pub name: String,
    /// Module's full path
    pub path: String,
}

/// General memory protection.
#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Protection(pub u8);

#[allow(missing_docs)]
impl Protection {
    pub const NONE: Self = Self(0b000);
    pub const R: Self = Self(0b100);
    pub const RW: Self = Self(0b110);
    pub const RWX: Self = Self(0b111);
    pub const RX: Self = Self(0b101);
    pub const W: Self = Self(0b010);
    pub const WX: Self = Self(0b011);
    pub const X: Self = Self(0b001);
}

impl Protection {
    /// Can read?
    #[inline]
    pub fn read(&self) -> bool {
        self.0 & Self::R.0 != 0
    }

    /// Can write?
    #[inline]
    pub fn write(&self) -> bool {
        self.0 & Self::W.0 != 0
    }

    /// Can execute?
    #[inline]
    pub fn execute(&self) -> bool {
        self.0 & Self::X.0 != 0
    }

    /// Parses string of kind `r-x`.
    /// # Panics
    /// * `s.len()` isn't 3.
    /// * s[0] != r | -
    /// * s[1] != w | -
    /// * s[2] != x | -
    pub fn parse(s: &str) -> Self {
        assert!(
            s.len() == 3 &&
            s.is_ascii() &&
            s.chars().all(|c| c == '-' || c == 'r' || c == 'w' || c == 'x')
        );

        let val = ((s.as_bytes()[0] == b'r') as u8) << 2 |
            ((s.as_bytes()[1] == b'w') as u8) << 1 |
            (s.as_bytes()[2] == b'x') as u8;
        
        Self(val)
    }
}

#[test]
fn test_parse_prot() {
    assert_eq!(Protection::RX, Protection::parse("r-x"));
}

impl BitOr for Protection {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Protection {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for Protection {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Protection {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitXor for Protection {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Protection {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}
