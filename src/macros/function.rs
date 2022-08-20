use crate::cell::StaticCell;
use core::{marker::PhantomData, mem::transmute, ops::Deref};

/// Creates a function defenition with explicitly defined offset from module.
/// ```
/// memflex::function! {
///     extern "C" fn CALLE(i32, i32) -> i32 = "file.exe"#0x1122;
/// }
/// ```
#[macro_export]
macro_rules! function {
    (
        $(
            $(extern $($abi:literal)?)? fn $fname:ident( $($atype:ty),* ) $(-> $ret:ty)? = $($resolver:ident)? $modname:literal $sep:tt $offset:expr;
        )*
    ) => {
        $(
            static $fname: $crate::Function< $(extern $($abi)?)? fn($($atype),*) $(-> $ret)?> = $crate::Function::new(
                || unsafe {
                    ($crate::__resolver!( $($resolver)? ))($modname, $offset)
                }
            );
        )*
    };
}

#[doc(hidden)]
pub struct Function<F> {
    cell: StaticCell<usize, fn() -> usize>,
    _ph: PhantomData<F>,
}

impl<F> Function<F> {
    #[doc(hidden)]
    pub const fn new(init: fn() -> usize) -> Self {
        Self {
            _ph: PhantomData,
            cell: StaticCell::new(init),
        }
    }
}

impl<F> Deref for Function<F> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        unsafe { transmute(self.cell.value()) }
    }
}
