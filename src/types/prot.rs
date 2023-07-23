bitflags::bitflags! {
    /// Protection of a memory region.
    #[derive(Default)]
    pub struct Protection : u8 {
        /// Read
        const R = 0b001;
        /// Write
        const W = 0b010;
        /// Execute
        const X = 0b100;
        /// Read | Write
        const RW = 0b011;
        /// Read | Execute
        const RX = 0b101;
        /// Read | Write | Execute
        const RWX = 0b111;
    }
}

impl Protection {
    /// Can read?
    #[inline]
    pub const fn read(&self) -> bool {
        self.contains(Self::R)
    }

    /// Can write?
    #[inline]
    pub const fn write(&self) -> bool {
        self.contains(Self::W)
    }

    /// Can execute?
    #[inline]
    pub const fn execute(&self) -> bool {
        self.contains(Self::X)
    }

    /// Parses string of kind `r-x`.
    /// # Panics
    /// * `s.len()` isn't 3.
    /// * s\[0\] != r | -
    /// * s\[1\] != w | -
    /// * s\[2\] != x | -
    pub fn parse(s: &str) -> Self {
        assert!(
            s.len() == 3
                && s.is_ascii()
                && s.chars()
                    .all(|c| c == '-' || c == 'r' || c == 'w' || c == 'x')
        );

        let mut prot = Self::empty();
        if s.as_bytes()[0] == b'r' {
            prot |= Self::R;
        }

        if s.as_bytes()[1] == b'w' {
            prot |= Self::W;
        }

        if s.as_bytes()[2] == b'x' {
            prot |= Self::X;
        }

        prot
    }

    /// Converts from os protection type.
    #[cfg(all(windows, feature = "std"))]
    pub fn from_os(value: windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS) -> Option<Self> {
        use windows::Win32::System::Memory::{
            PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE, PAGE_NOACCESS, PAGE_READONLY, PAGE_READWRITE,
        };

        match value {
            PAGE_NOACCESS => Some(Self::empty()),
            PAGE_READONLY => Some(Self::R),
            PAGE_READWRITE => Some(Self::RW),
            PAGE_EXECUTE_READWRITE => Some(Self::RWX),
            PAGE_EXECUTE_READ => Some(Self::RX),
            _ => None,
        }
    }

    /// Converts to os protection type.
    #[cfg(all(windows, feature = "std"))]
    pub fn to_os(&self) -> windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS {
        use windows::Win32::System::Memory::{
            PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE, PAGE_NOACCESS, PAGE_READONLY, PAGE_READWRITE,
        };

        match (self.read(), self.write(), self.execute()) {
            (true, false, false) => PAGE_READONLY,        // Self::R
            (true, false, true) => PAGE_EXECUTE_READ,     // Self::RX
            (true, true, false) => PAGE_READWRITE,        // Self::RW
            (true, true, true) => PAGE_EXECUTE_READWRITE, // Self:RWX
            (false, _, _) => PAGE_NOACCESS,
        }
    }

    /// Converts to os protection type.
    #[cfg(all(unix, feature = "std"))]
    pub const fn to_os(&self) -> i32 {
        self.bits() as i32
    }

    /// Converts from os protection type.
    #[cfg(all(unix, feature = "std"))]
    pub const fn from_os(prot: i32) -> Self {
        Self::from_bits_truncate(prot as u8)
    }
}
