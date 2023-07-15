/// Creates a structure with access to individual bits
/// ```
/// memflex::bitstruct! {
///     struct SomeStruct : u8 {
///         // Bits: 0, 1, 2
///         first: 0..=2,
///         // Bits: 3, 4, 5, 6, 7
///         second: 3..=7,
///     }
/// }
///
/// let mut s = SomeStruct { bits: 0b11011101 };
/// // Bits:    | 1 1 0 1 1 | 1 0 1 |
/// // Index:   | 7 6 5 4 3 | 2 1 0 |
/// // Span:    |   3..=7   | 0..=2 |
/// // Names:   |  Second   | First |
/// assert_eq!(s.first(), 0b101);
/// assert_eq!(s.second(),  0b11011);
/// s.set_first(0b010);
/// assert_eq!(s.first(), 0b010);
/// assert_eq!(s.second(),  0b11011);
/// s.set_second(0b10101);
/// assert_eq!(s.second(), 0b10101);
/// assert_eq!(s.first(), 0b010);
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
            $( #[$($outter)*] )*
            #[repr(transparent)]
            $vs struct $sname {
                pub bits: $int
            }

            #[allow(dead_code)]
            impl $sname {
                /// Creates new bitstruct
                pub fn new(bits: $int) -> Self {
                    Self { bits }
                }

                $(
                    $fvs fn $fname(&self) -> $int {
                        (self.bits >> $from) & !(!0 << ($to - $from + 1))
                    }

                    $crate::paste! {
                        $fvs fn [<set_ $fname>](&mut self, value: $int) {
                            let mask: $int = !0 << $to - $from + 1;
                            let v = mask.rotate_left($from);
                            self.bits = (self.bits & v) | ((value & !mask) << $from);
                        }
                    }
                )*
            }

            impl core::convert::From<$sname> for $int {
                fn from(v: $sname) -> Self {
                    v.bits
                }
            }

            impl core::convert::From<$int> for $sname {
                fn from(bits: $int) -> Self {
                    Self { bits }
                }
            }
        )*
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bitstruct() {
        crate::bitstruct! {
            struct Foo : u8 {
                a: 0..=4,
                b: 5..=7,
                c: 3..=3,
                d: 4..=4,
            }
        }

        let a = Foo::new(0b10101010);
        assert_eq!(a.a(), 0b1010);
        assert_eq!(a.b(), 0b101);
        assert_eq!(a.c(), 1);
        assert_eq!(a.d(), 0);
    }
}
