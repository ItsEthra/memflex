/// Creates a structure with access to individual bits
/// ```
/// memflex::bitstruct! {
///     struct SomeStruct : u8 {
///         // Bits: 0, 1, 2
///         first: 0..=2,
///         // Bits: 3, 4, 5, 6, 7
///         next: 3..=7,
///     }
/// }
/// 
/// let s = SomeStruct { bits: 0b11011101 };
/// // Bits:    | 1 1 0 1 1 | 1 0 1 |
/// // Index:   | 7 6 5 4 3 | 2 1 0 |
/// // Values:  | 3..=7     | 0..=2 |
/// assert_eq!(s.first(), 0b101);
/// assert_eq!(s.next(),  0b11011);
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

            impl $sname {
                /// Creates new bitstruct
                pub fn new(bits: $int) -> Self {
                    Self { bits }
                }

                $(
                    $fvs fn $fname(&self) -> $int {
                        (self.bits >> $from) & (<$int>::MAX >> (<$int>::BITS - ($to - $from + 1)))
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
