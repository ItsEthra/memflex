use core::marker::PhantomData;

/// Creates a structure with access to individual bits
/// ```
/// memflex::bitstruct! {
///     pub struct Foo : u8 {
///         a: 0..=3,
///         b: 4..=6,
///         c: 7..=7
///     }
/// }
///
/// let foo = Foo::from_bits(0b_1101_1101);
/// assert_eq!(foo.a().get(), 0b_1101);
/// assert_eq!(foo.b().get(), 0b_101);
/// assert_eq!(foo.c().as_bool(), true);
/// foo.a().set(0b_0110);
/// foo.b().set(0b_101);
/// foo.c().set_bool(false);
/// assert_eq!(foo.a().get(), 0b_0110);
/// assert_eq!(foo.b().get(), 0b_101);
/// assert_eq!(foo.c().as_bool(), false);
/// assert_eq!(foo.bits(), 0b_0101_0110);
/// ```
#[macro_export]
macro_rules! bitstruct {
    {
        $(
            $( #[$($outter:tt)*] )*
            $vs:vis struct $sname:ident : $int:ty {
                $(
                    $fvs:vis $fname:ident: $from:tt..=$to:tt
                ),*$(,)?
            }
        )*
    } => {
        $(

            #[repr(transparent)]
            struct $sname(core::cell::UnsafeCell<$int>);

            unsafe impl Sync for $sname {}

            #[allow(dead_code)]
            impl $sname {
                #[inline]
                pub fn bits(&self) -> $int {
                    unsafe { *self.0.get() }
                }

                #[inline]
                pub fn from_bits(bits: $int) -> Self {
                    Self(core::cell::UnsafeCell::new(bits))
                }

                $(
                    $fvs fn $fname(&self) -> $crate::BitField<$int, {$from % 8}, {$to - $from + 1}> {
                        let x = if $from % 8 == 0 && $from != 0 {
                            $from / 8 + 1
                        } else {
                            $from / 8
                        };
                        let ptr = unsafe { self.0.get().cast::<u8>().add(x) };

                        unsafe { $crate::BitField::from_ptr(ptr) }
                    }
                )*
            }
        )*
    }
}

#[doc(hidden)]
pub trait BitInteger {
    fn shr(self, v: usize) -> Self;
    fn mask(self, v: usize) -> Self;
    fn ror(self, v: usize) -> Self;
    fn rol(self, v: usize) -> Self;
    fn set(self, v: Self, l: usize) -> Self;
}

macro_rules! impl_int {
    ($($int:ty),*) => {
        $(
            impl BitInteger for $int {
                #[inline(always)]
                fn shr(self, v: usize) -> Self {
                    self >> v
                }

                #[inline(always)]
                fn mask(self, v: usize) -> Self {
                    self & !((!0 as $int).checked_shl(v as u32).unwrap_or(0))
                }

                #[inline(always)]
                fn ror(self, v: usize) -> Self {
                    self.rotate_right(v as u32)
                }

                #[inline(always)]
                fn rol(self, v: usize) -> Self {
                    self.rotate_left(v as u32)
                }

                #[inline(always)]
                fn set(self, v: $int, l: usize) -> Self {
                    let mask = !((!0 as $int).checked_shl(l as u32).unwrap_or(0));
                    (self & !mask) | (v & mask)
                }
            }
        )*
    };
}
impl_int!(u8, u16, u32, u64, u128);

/// Implement bitfields for an existing type.
#[macro_export]
macro_rules! bitfields {
    {
        $(
            $target:ident.$field:ident: $int:ident {
                $(
                    $fvs:vis $fname:ident: $from:tt..=$to:tt
                ),*$(,)?
            }
        )*
    } => {
        $(
            impl $target {
                $(
                    $fvs fn $fname(&self) -> $crate::BitField<$int, {$from % 8}, {$to - $from + 1}> {
                        let x = if $from % 8 == 0 && $from != 0 {
                            $from / 8 + 1
                        } else {
                            $from / 8
                        };
                        let ptr = unsafe { core::ptr::addr_of!(self.$field).cast_mut().cast::<u8>().add(x) };

                        unsafe { $crate::BitField::from_ptr(ptr) }

                    }
                )*
            }
        )*
    };
}

/// Racy bitfield that provides `get`, `set` methods.
pub struct BitField<I: BitInteger, const O: usize, const L: usize> {
    ptr: *mut u8,
    pd: PhantomData<I>,
}

impl<I: BitInteger, const O: usize, const L: usize> BitField<I, O, L> {
    /// Creates new BitField from a pointer.
    pub const unsafe fn from_ptr(ptr: *mut u8) -> Self {
        Self {
            pd: PhantomData,
            ptr,
        }
    }

    /// Returns the value of this bitfield.
    #[inline]
    pub fn get(&self) -> I {
        let mut val = unsafe { self.ptr.cast::<I>().read_unaligned() };
        val = val.shr(O).mask(L);
        val
    }

    /// Assigns new value to the bitfield.
    #[inline]
    pub fn set(&self, value: I) {
        let mut val = unsafe { self.ptr.cast::<I>().read_unaligned() };
        val = val.ror(O).set(value, L).rol(O);
        unsafe { self.ptr.cast::<I>().write_unaligned(val) };
    }
}

impl<I: BitInteger, const O: usize> BitField<I, O, 1> {
    /// Converts bitfield to a bool value.
    #[inline]
    pub fn as_bool(&self) -> bool {
        unsafe { (self.ptr.read() >> O) & 1 != 0 }
    }

    /// Sets new value for a bit field.
    #[inline]
    pub fn set_bool(&self, value: bool) {
        unsafe {
            let new =
                ((self.ptr.read().rotate_right(O as u32) & !1) | value as u8).rotate_left(O as u32);
            self.ptr.write(new);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{bitstruct, BitField};

    bitstruct! {
        pub struct Foo : u16 {
            a: 0..=3,
            b: 4..=11,
            c: 12..=15
        }
    }

    struct Bar {
        pad: u16,
        bitfield: u8,
    }

    bitfields! {
        Bar.bitfield: u8 {
            a: 0..=3,
            b: 4..=6,
            c: 7..=7
        }
    }

    #[test]
    fn test_bitfield_macro() {
        let bar = Bar {
            pad: 0,
            bitfield: 0b_1100_1010,
        };
        assert_eq!(bar.a().get(), 0b_1010);
        assert_eq!(bar.b().get(), 0b_100);
        assert_eq!(bar.c().as_bool(), true);
        bar.a().set(0b_0011);
        bar.b().set(0b_011);
        bar.c().set_bool(false);
        assert_eq!(bar.a().get(), 0b_0011);
        assert_eq!(bar.b().get(), 0b_011);
        assert_eq!(bar.c().as_bool(), false);
        assert_eq!(bar.pad, 0);
        assert_eq!(bar.bitfield, 0b_0011_0011);
    }

    #[test]
    fn test_bitstruct_macro() {
        let foo = Foo::from_bits(0b_11111111_00001100);
        assert_eq!(foo.a().get(), 0b_1100);
        assert_eq!(foo.b().get(), 0b_1111_0000);
        assert_eq!(foo.c().get(), 0b_1111);
        foo.a().set(0b_0011);
        foo.b().set(0b_0000_1111);
        foo.c().set(0b_0010);
        assert_eq!(foo.a().get(), 0b_0011);
        assert_eq!(foo.b().get(), 0b_0000_1111);
        assert_eq!(foo.c().get(), 0b_0010);

        assert_eq!(foo.bits(), 0b_00100000_11110011);
    }

    #[test]
    fn test_bitstruct_multi() {
        let mut byte = 0b_11111111_00001100;
        let f1 = unsafe { BitField::<u16, 4, 8>::from_ptr(&mut byte as *mut _ as _) };
        let f2 = unsafe { BitField::<u16, 0, 4>::from_ptr(&mut byte as *mut _ as _) };
        assert_eq!(f1.get(), 0b_1111_0000);
        assert_eq!(f2.get(), 0b_1100);
        f1.set(0b_0000_1111);
        f2.set(0b_0011);
        assert_eq!(f1.get(), 0b_0000_1111);
        assert_eq!(f2.get(), 0b_0011);
        assert_eq!(byte, 0b_11110000_11110011);
    }

    #[test]
    fn test_bitstruct_bool() {
        let mut byte = 0b_10101010;
        let f1 = unsafe { BitField::<u8, 3, 1>::from_ptr(&mut byte as *mut _ as _) };
        let f2 = unsafe { BitField::<u8, 4, 1>::from_ptr(&mut byte as *mut _ as _) };
        assert!(f1.as_bool());
        assert!(!f2.as_bool());
        f1.set_bool(false);
        f2.set_bool(true);
        assert!(!f1.as_bool());
        assert!(f2.as_bool());
        assert_eq!(byte, 0b_10110010);
    }
}
